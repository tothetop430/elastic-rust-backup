#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate slog_stdlog;
extern crate slog_envlogger;

use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::slice::Iter;

#[derive(Deserialize, Debug)]
pub struct Response {
    took: u64,
    timed_out: bool,
    _shards: Shards,
    hits: Hits,
    aggregations: Option<Aggregations>,
    status: Option<u16>
}

#[derive(Deserialize, Debug)]
pub struct Aggregations(Value);


//QUESTION: Why do lifetimes go where they go, what am I missing?
//          What happens when lifetimes shadow each other?
//          Mental model for this?
//          Below works thanks to `misdreavus` on IRC, but I don't quite know why
impl<'a> IntoIterator for &'a Aggregations {
    type Item = BTreeMap<Cow<'a, String>, &'a Value>; // JPG - str?
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

//FIXME: can this be run as a state-machine (ala https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
//#[derive(Debug)]
//enum AggStates {
//    AtRoot,
//    AtName,
//    InBuckets,
//    AtValues,
//    RowFinished
//}

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    current_row: Option<BTreeMap<Cow<'a, String>, &'a Value>>,
    current_row_finished: bool,
    //QUESTION: Tracking traversal usng a stack of Iterators make sense? Is Vec right for this?
    iter_stack: Vec<(Option<&'a String>, Iter<'a, Value>)>,
    aggregations: &'a Aggregations
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {
        let v = match a {
            &Aggregations(ref v) => v
        }; // JPG if let, destructure, direct

        // JPG: inference?
        let mut s: Vec<(Option<&String>, Iter<Value>)> = Vec::new();

        match *v {
            Value::Object(ref o) => {
                for (key, child) in o {
                    if let Value::Object(ref c) = *child {
                        if c.contains_key("buckets") { // JPG: entry
                            if let Value::Array(ref a) = c["buckets"] {
                                let i = a.iter();
                                s.push((Some(&key), i));
                            }
                        }
                    }
                }
            },
            _ => {
                //FIXME: Bad for lib
                panic!("Not implemented, we only cater for bucket objects");
            } // quick-error
        };

        AggregationIterator {
            current_row: None,
            current_row_finished: false,
            iter_stack: s,
            aggregations: a
        }
    }
}

macro_rules! insert_value {
    ($fieldname:expr, $json_object:ident, $keyname:ident, $rowdata:ident) => ({
      if $json_object.contains_key($fieldname) {
          let v = $json_object.get($fieldname);
          let field_name = format!("{}_{}", $keyname, $fieldname);
          debug! ("ITER: Insert value! {} {:?}", field_name, v.unwrap());
          $rowdata.insert(Cow::Owned(field_name), v.unwrap());
       }
	})
}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = BTreeMap<Cow<'a, String>, &'a Value>; // JPG type alias?

    fn next(&mut self) -> Option<BTreeMap<Cow<'a, String>, &'a Value>> {
        match self.current_row {
            None => {
                //New row
                self.current_row = Some(BTreeMap::new())
            },
            Some(_) => () // JPG if let
        };

        loop {
            match self.iter_stack.pop() {
                None => {
                    debug! ("ITER: Done!"); // JPG: no space on macro invocation
                    match self.current_row { // JPG always set to None.
                        Some(_) => {
                            self.current_row = None
                        },
                        _ => ()
                    };
                    break;
                },
                Some(mut i) => {
                    let n = i.1.next(); // JPG tuples used like this are hard to follow
                    //FIXME: can this fail?
                    let active_name = &i.0.unwrap();

                    //Iterate down?
                    let mut has_buckets = false;
                    //Save
                    self.iter_stack.push(i);

                    debug! ("ITER: Depth {}", self.iter_stack.len());
                    //FIXME: Move this, to be able to process first line too
                    match n {
                        None => {
                            //Was nothing here, exit
                            debug! ("ITER: Exit!");
                            self.iter_stack.pop();
                            continue;
                        },
                        Some(n) => {
                            //QUESTION: Destructuring/matching to this extent the right strategy?
                            match self.current_row {
                                Some(ref mut row) => {
                                    debug! ("ITER: Row: {:?}", row);

                                    let o = match *n { // JPG as_object
                                        Value::Object(ref o) => o,
                                        _ => panic!("Shouldn't get here!")
                                    };
                                    for (key, value) in o {
                                        match *value {
                                            Value::Object(ref c) => {
                                                //Child Aggregation
                                                if c.contains_key("buckets") {
                                                    has_buckets = true;
                                                    if let Value::Array(ref a) = c["buckets"] {
                                                        let i = a.iter();
                                                        self.iter_stack.push((Some(key), i));
                                                    }
                                                    continue;
                                                }
                                                //Simple Value Aggregation Name
                                                if c.contains_key("value") {
                                                    let v = c.get("value");
                                                    //FIXME: Can this fail?
                                                    debug! ("ITER: Insert value! {} {:?}", key, v.unwrap());
                                                    //QUESTION: Cow right for this use-case? See below
                                                    row.insert(Cow::Borrowed(key), v.unwrap());
                                                    continue;
                                                }
                                                //Stats
                                                //FIXME: Can be done in loop?

                                                //Stats fields
                                                insert_value!("count",c,key,row);
                                                insert_value!("min",c,key,row);
                                                insert_value!("max",c,key,row);
                                                insert_value!("avg",c,key,row);
                                                insert_value!("sum",c,key,row);
                                                insert_value!("sum_of_squares",c,key,row);
                                                insert_value!("variance",c,key,row);
                                                insert_value!("std_deviation",c,key,row);

                                                //TODO: std_deviation_bounds
//                                                if c.contains_key("std_deviation_bounds") {
//                                                    let u = c.get("std_deviation_bounds").unwrap().get("upper");
//                                                    let l = c.get("std_deviation_bounds").unwrap().get("lower");
//                                                    let un = format!("{}_std_deviation_bounds_upper", key);
//                                                    let ln = format!("{}_std_deviation_bounds_lower", key);
//                                                    debug! ("ITER: Insert std_dev_bounds! {} {} u: {:?} l: {:?}", un, ln, u.unwrap(), l.unwrap());
//                                                    row.insert(Cow::Owned(un), u.unwrap());
//                                                    row.insert(Cow::Owned(ln), l.unwrap());
//                                                }
                                            },
                                            _ => ()
                                        }
                                        //Bucket Aggregation Name
                                        if key == "key" {
                                            debug! ("ITER: Insert bucket! {} {:?}", active_name, value);
                                            row.insert(Cow::Borrowed(active_name), value);
                                        }
                                        //Bucket Aggregation Count
                                        if key == "doc_count" {
                                            debug! ("ITER: Insert bucket count! {} {:?}", active_name, value);
                                            let field_name = format!("{}_doc_count", active_name);
                                            row.insert(Cow::Owned(field_name), value);
                                        }
                                    }
                                    //Return here?
                                    if !has_buckets {
                                        return Some(row.clone());
                                    }
                                },
                                _ => ()
                            }
                        }
                    }

                    if !has_buckets {
                        debug! ("ITER: Bucketless!");
                        break;
                    } else {
                        debug! ("ITER: Dive!");
                    }
                }
            };
        }

        match self.current_row {
            //FIXME: Refactor to avoid this clone()
            Some(ref x) => Some(x.clone()),
            None => None
        }
    }
}

#[derive(Deserialize, Debug)]
struct Shards {
    total: u32,
    successful: u32,
    failed: u32
}

#[derive(Deserialize, Debug)]
pub struct Hits {
    total: u64,
    max_score: u64,
    hits: Vec<Value>
}

// JPG place impls closer to defns
impl Response {
    pub fn hits(&self) -> &Vec<Value> {
        &self.hits.hits()
    }

    pub fn aggs(self) -> std::option::Option<Aggregations> {
        self.aggregations
    }
}

impl Hits {
    pub fn hits(&self) -> &Vec<Value> { // JPG http://stackoverflow.com/q/40006219/155423
        &self.hits
    }
}

#[derive(Deserialize, Debug)]
struct Hit {
    _index: String // JPG suspicious
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

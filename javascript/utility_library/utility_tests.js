var $ol = document.querySelector("ol");

function outputResult(message) {
  var $li = document.createElement("li");
  $li.innerText = message;
  $ol.appendChild($li);
  return $li;
}

function test(message, assertion) {
  var $msg = outputResult(message),
      passed = false;

  try {
    passed = assertion();
  }
  catch (e) {
    passed = false;
  }
  $msg.setAttribute("class", passed ? "pass" : "fail");
}

test("_ is defined", function() {
  return typeof _ !== "undefined";
});

// _.first
test("first is defined", function() {
  return typeof _().first === "function";
});
test("first returns first element in an array", function() {
  return _([4]).first() === 4;
});
(function() {
  var a = [];
  a[1] = 4;
  test("first does not return second element even if first is undefined", function() {
    return _(a).first() === undefined;
  });
})();

// _.last
test("last is defined", function() {
  return typeof _().last === "function";
});
test("last returns last element in an array", function() {
  return _([1, 2, 3, 4]).last() === 4;
});

// _.without
test("without is defined", function() {
  return typeof _().without === "function";
});
test("without returns new array that does not contain the supplied value", function() {
  return _([1, 2, 3, 4, 5]).without(4).indexOf(4) === -1;
});
test("without removes any number of arguments", function() {
  var a = _([1, 2, 3, 4, 5, 6]).without(6, 4);
  return a.indexOf(6) === -1 && a.indexOf(4) === -1;
});
test("without removes only the specified number of arguments", function() {
  var a = _([1, 2, 3, 4, 5, 6]).without(6, 4);
  return a.length === 4;
});
test("without retains the elements that aren't remove", function() {
  var a = _([1, 2, 3, 4, 5, 6]).without(6, 4);
  return a.indexOf(1) === 0 && a.indexOf(2) === 1 && a.indexOf(3) === 2 && a.indexOf(5) === 3;
});

// _.range
test("range is defined", function() {
  return typeof _.range === "function";
});
test("range returns an array of values starting at 0 when one argument supplied", function() {
  return _.range(10)[0] === 0 && _.range(10).length === 10;
});
test("range returns an array of values starting with the first value when two arguments supplied", function() {
  return _.range(1, 10)[0] === 1 && _.range(1, 10).length === 9;
});

// _.lastIndexOf
test("lastIndexOf is defined", function() {
  return typeof _().lastIndexOf === "function";
});
test("lastIndexOf returns last index of supplied value", function() {
  return _([1, 1, 1]).lastIndexOf(1) === 2 && _([1, 2, 3]).lastIndexOf(2) === 1;
});

// _.sample
test("sample is defined", function() {
  return typeof _().sample === "function";
});
test("sample returns a single element when no argument supplied", function() {
  return _([1]).sample() === 1;
});
test("sample returns multiple, non-repetitive elements when a numeric argument supplied", function() {
  return _([1, 2, 3]).sample(3).length === 3;
});

// _.findWhere
test("findWhere is defined", function() {
  return typeof _().findWhere === "function";
});
(function() {
  var dict = [{ foo: "bar", idx: 0 }, { foo: "baz", idx: 1 }, { foo: "bar", idx: 2 }];

  test("findWhere returns the first object with matched properties", function() {
    return _(dict).findWhere({ foo: "bar" }).idx === 0;
  });
})();
(function() {
  var dict = [{ foo: "bar", quux: "q", idx: 0 }, { foo: "baz", quux: "z", idx: 1 }, { foo: "bar", quux: "z", idx: 2 }];

  test("findWhere returns the first object with multiple matched properties", function() {
    return _(dict).findWhere({ foo: "bar", quux: "z" }).idx === 2;
  });
})();
(function() {
  var dict = [{ foo: "bar", idx: 0 }, { foo: "baz", idx: 1 }, { foo: "bar", idx: 2 }];

  test("findWhere returns undefined with no matched properties", function() {
    return _(dict).findWhere({ foo: "quux" }) === undefined;
  });
})();

// _.where
test("where is defined", function() {
  return typeof _().where === "function";
});

(function() {
  var dict = [{ foo: "bar", idx: 0 }, { foo: "baz", idx: 1 }, { foo: "bar", idx: 2 }];

  test("where returns an array with one matched object", function() {
    return _(dict).where({ idx: 0 }).length === 1;
  });
  test("where returns an array with two matched objects", function() {
    return _(dict).where({ foo: "bar" }).length === 2;
  });
})();

// _.pluck
test("pluck is defined", function() {
  return typeof _().pluck === "function";
});

test("pluck returns array of two values", function() {
  var coll = [{ foo: "bar" }, { foo: "baz" }],
      pluck = _(coll).pluck("foo");

  return pluck.length === 2;
});
test("pluck returns both values", function() {
  var coll = [{ foo: "bar" }, { foo: "baz" }],
      pluck = _(coll).pluck("foo");

  return pluck[0] === "bar" && pluck[1] === "baz";
});

// _.keys
test("keys is defined", function() {
  return typeof _().keys === "function";
});

test("keys returns an array of keys from the object", function() {
  var keys = _({ foo: "bar", baz: "quuz" }).keys();
  return keys.length === 2;
});
test("keys returns all keys that are own properties of the object", function() {
  var keys = _({ foo: "bar", baz: "quuz" }).keys();
  return keys.indexOf("foo") !== -1 && keys.indexOf("baz") !== -1;
});
test("keys does not return inherited object properties", function() {
  var keys = _({ foo: "bar", baz: "quuz" }).keys();
  return keys.indexOf("toString") === -1;
});

// _.values
test("values is defined", function() {
  return typeof _().values === "function";
});

test("values returns an array of values from the object", function() {
  var values = _({ foo: "bar", baz: "quuz" }).values();
  return values.length === 2;
});
test("values returns all values that are own properties of the object", function() {
  var values = _({ foo: "bar", baz: "quuz" }).values();
  return values.indexOf("bar") !== -1 && values.indexOf("quuz") !== -1;
});

// _.extend
test("extend is defined", function() {
  return typeof _.extend === "function";
});

test("extend returns an extended object using new object's values", function() {
  var new_obj = { bar: "baz" },
      old_obj = { foo: "bar" },
      ext_obj = _.extend(old_obj, new_obj);
      crazy_object = _.extend({ foo: "quuz" }, new_obj, old_obj);
  return ext_obj.foo === "bar" && ext_obj.bar === "baz";
});
test("extend modifies the first object passed in rather than creating a new object", function() {
  var new_obj = { bar: "baz" },
      old_obj = { foo: "bar" },
      ext_obj = _.extend(old_obj, new_obj);
      crazy_object = _.extend({ foo: "quuz" }, new_obj, old_obj);
  return new_obj === _.extend(new_obj, {});
});
test("extend works with any number of objects", function() {
  var new_obj = { bar: "baz" },
      old_obj = { foo: "bar" },
      ext_obj = _.extend(old_obj, new_obj);
      crazy_object = _.extend({ foo: "quuz" }, new_obj, old_obj);
  return crazy_object.foo === "bar";
});

// _.pick
test("pick is defined", function() {
  return typeof _().pick === "function";
});

test("pick returns a new object with the passed in properties", function() {
  var old_obj = { foo: "bar" },
      new_obj = _(old_obj).pick("foo");

  return new_obj.foo === old_obj.foo && new_obj !== old_obj;
});
test("pick ignores any properties passed in that do not exist on the source object", function() {
  var old_obj = { foo: "bar" },
      new_obj = _(old_obj).pick("foo");

  return _(new_obj).pick("foo", "bar").bar === undefined;
});

// _.omit
test("omit is defined", function() {
  return typeof _().omit === "function";
});

test("omit returns a new object with any passed in properties omitted", function() {
  var old_obj = { foo: "bar" },
      new_obj = _(old_obj).omit("foo");

  return new_obj.foo === undefined;
});

test("omit doesn't insert properties that aren't on the original object", function() {
  var old_obj = { foo: "bar" },
      new_obj = _(old_obj).omit("foo", "foo2");

  return new_obj.hasOwnProperty('foo2') === false;
});

test("omit doesn't remove all the properties", function() {
  var old_obj = { foo: "bar", foo2: "bar2" },
      new_obj = _(old_obj).omit("foo");

  return new_obj.hasOwnProperty('foo') === false && new_obj.hasOwnProperty('foo2') === true;
});

// _.has
test("has is defined", function() {
  return typeof _().has === "function";
});

test("has returns true when property exists", function() {
  var o = { foo: "bar" };

  return _(o).has("foo");
});
test("has returns false when property does not exist", function() {
  var o = { foo: "bar" };

  return !_(o).has("bar");
});
test("has returns true when hasOwnProperty is defined", function() {
  var o = { foo: "bar" };
  o.hasOwnProperty = function() { };

  return _(o).has("hasOwnProperty");
});

(["isElement", "isArray", "isObject", "isFunction", "isBoolean", "isString", "isNumber"]).forEach(function(method) {
  test(method + " is defined", function() {
    return typeof _[method] === "function" && typeof _()[method] === "function";
  });
});
test("isElement returns true if DOM element, otherwise false", function() {
  return _.isElement(document.body) && !_.isElement({});
});
test("isArray returns true if array, otherwise false", function() {
  return _.isArray([]) && !_.isArray({ 0: "a", 1: "b" });
});
test("isObject returns true if object or function, otherwise false", function() {
  return _.isObject({}) && _.isObject([]) && _.isObject(isNaN) && !_.isObject(1);
});
test("isFunction returns true if function, otherwise false", function() {
  return _.isFunction(isNaN) && !_.isFunction({});
});
test("isBoolean returns true if boolean (primitive or object), otherwise false", function() {
  return _.isBoolean(false) && _.isBoolean(new Boolean(false)) && !_.isBoolean(1);
});
test("isString returns true if string, otherwise false", function() {
  return _.isString("") && _.isString(new String()) && !_.isString(1);
});
test("isNumber returns true if number, (primitive or object), otherwise false", function() {
  return _.isNumber(1) && _.isNumber(new Number(5)) && !_.isNumber("5");
});

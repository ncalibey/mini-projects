var _ = function (element) {
  var u = {
    first: function () {
      return element[0];
    },

    last: function () {
      return element[element.length - 1];
    },

    without: function () {
      var values = Array.prototype.slice.call(arguments);

      return element.filter(function (e) {
        return values.indexOf(e) === -1;
      });
    },

    lastIndexOf: function (value) {
      var idx = -1;
      var i;

      for (i = element.length - 1; i >= 0; i -= 1) {
        if (element[i] === value) {
          idx = i;
          break;
        }
      }

      return idx;
    },

    sample: function (qty) {
      var sampled = [];
      var copy = element.slice();
      var index;

      qty = qty || 1;

      while (sampled.length !== qty) {
        index = Math.floor(Math.random() * qty);

        sampled.push(copy.splice(index, 1)[0]);
      }

      return qty === 1 ? sampled[0] : sampled;
    },

    findWhere: function (obj) {
      var keys = Object.keys(obj);
      var match;
      var i;

      for (i = 0; i < element.length; i += 1) {
        match = keys.every(function (key) {
          return obj[key] === element[i][key];
        });

        if (match) return element[i];
      }
    },

    where: function (obj) {
      var keys = Object.keys(obj);
      var match;

      return element.filter(function (object) {
        return match = keys.every(function (key) {
          return obj[key] === object[key];
        });
      });
    },

    pluck: function (key) {
      return element.map(function (obj) {
        return obj[key];
      });
    },

    keys: function () {
      return Object.getOwnPropertyNames(element);
    },

    values: function () {
      return Object.getOwnPropertyNames(element).map(function (key) {
        return element[key];
      });
    },

    pick: function () {
      var keys = Array.prototype.slice.call(arguments);
      var newObj = {};

      keys.forEach(function (key) {
        if (element.hasOwnProperty(key)) {
          newObj[key] = element[key];
        }
      });

      return newObj;
    },

    omit: function () {
      var keys = Array.prototype.slice.call(arguments);
      var newObj = {};
      var prop;

      for (prop in element) {
        if (!keys.includes(prop)) {
          newObj[prop] = element[prop];
        }
      }

      return newObj;
    },

    has: function (property) {
      return Object.keys(element).includes(property);
    },
  };

  ['isElement', 'isArray', 'isObject', 'isFunction', 'isBoolean', 'isString',
   'isNumber'].forEach(function (method) {
     u[method] = function () { _[method].call(u, element); }
   });

  return u;
}

_.range = function (start, stop) {
  var range = [];

  if (stop === undefined) {
    stop = start;
    start = 0;
  }

  for (i = start; i < stop; i += 1) {
    range.push(i);
  }

  return range;
};

_.extend = function () {
  var args = Array.prototype.slice.call(arguments)
  var last;
  var secondToLast;
  var key;

  while (args.length > 1) {
    last = args[args.length - 1];
    secondToLast = args[args.length - 2];
    key;

    for (key in last) {
      secondToLast[key] = last[key];
    }

    args = args.slice(0, -1);
  }

  return args[0];
};

_.isElement = function (obj) {
  return obj instanceof Element;
};

_.isArray = function (obj) {
  return Array.isArray(obj);
};

_.isObject = function (obj) {
  return obj instanceof Object;
};

_.isFunction = function (func) {
  return func instanceof Function;
};

_.isBoolean = function (boolean) {
  return boolean instanceof Boolean || typeof boolean === 'boolean';
};

_.isString = function (string) {
  return string instanceof String || typeof string === 'string';
};

_.isNumber = function (number) {
  return number instanceof Number || typeof number === 'number';
};

(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./src/clumsy_web.js":
/*!***************************!*\
  !*** ./src/clumsy_web.js ***!
  \***************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.evaluate = evaluate;
exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;
exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;
exports.__wbindgen_string_new = __wbindgen_string_new;
exports.__wbindgen_number_new = __wbindgen_number_new;
exports.__wbindgen_number_get = __wbindgen_number_get;
exports.__wbindgen_undefined_new = __wbindgen_undefined_new;
exports.__wbindgen_null_new = __wbindgen_null_new;
exports.__wbindgen_is_null = __wbindgen_is_null;
exports.__wbindgen_is_undefined = __wbindgen_is_undefined;
exports.__wbindgen_boolean_new = __wbindgen_boolean_new;
exports.__wbindgen_boolean_get = __wbindgen_boolean_get;
exports.__wbindgen_symbol_new = __wbindgen_symbol_new;
exports.__wbindgen_is_symbol = __wbindgen_is_symbol;
exports.__wbindgen_string_get = __wbindgen_string_get;
exports.__wbindgen_throw = __wbindgen_throw;

var wasm = _interopRequireWildcard(__webpack_require__(/*! ./clumsy_web_bg */ "./src/clumsy_web_bg.wasm"));

function _interopRequireWildcard(obj) { if (obj && obj.__esModule) { return obj; } else { var newObj = {}; if (obj != null) { for (var key in obj) { if (Object.prototype.hasOwnProperty.call(obj, key)) { var desc = Object.defineProperty && Object.getOwnPropertyDescriptor ? Object.getOwnPropertyDescriptor(obj, key) : {}; if (desc.get || desc.set) { Object.defineProperty(newObj, key, desc); } else { newObj[key] = obj[key]; } } } } newObj.default = obj; return newObj; } }

function _slicedToArray(arr, i) { return _arrayWithHoles(arr) || _iterableToArrayLimit(arr, i) || _nonIterableRest(); }

function _nonIterableRest() { throw new TypeError("Invalid attempt to destructure non-iterable instance"); }

function _iterableToArrayLimit(arr, i) { var _arr = []; var _n = true; var _d = false; var _e = undefined; try { for (var _i = arr[Symbol.iterator](), _s; !(_n = (_s = _i.next()).done); _n = true) { _arr.push(_s.value); if (i && _arr.length === i) break; } } catch (err) { _d = true; _e = err; } finally { try { if (!_n && _i["return"] != null) _i["return"](); } finally { if (_d) throw _e; } } return _arr; }

function _arrayWithHoles(arr) { if (Array.isArray(arr)) return arr; }

function _typeof(obj) { if (typeof Symbol === "function" && typeof Symbol.iterator === "symbol") { _typeof = function _typeof(obj) { return typeof obj; }; } else { _typeof = function _typeof(obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj; }; } return _typeof(obj); }

var TextEncoder = (typeof self === "undefined" ? "undefined" : _typeof(self)) === 'object' && self.TextEncoder ? self.TextEncoder : __webpack_require__(/*! util */ "./node_modules/util/util.js").TextEncoder;
var cachedEncoder = new TextEncoder('utf-8');
var cachedUint8Memory = null;

function getUint8Memory() {
  if (cachedUint8Memory === null || cachedUint8Memory.buffer !== wasm.memory.buffer) cachedUint8Memory = new Uint8Array(wasm.memory.buffer);
  return cachedUint8Memory;
}

function passStringToWasm(arg) {
  var buf = cachedEncoder.encode(arg);

  var ptr = wasm.__wbindgen_malloc(buf.length);

  getUint8Memory().set(buf, ptr);
  return [ptr, buf.length];
}

var cachedUint32Memory = null;

function getUint32Memory() {
  if (cachedUint32Memory === null || cachedUint32Memory.buffer !== wasm.memory.buffer) cachedUint32Memory = new Uint32Array(wasm.memory.buffer);
  return cachedUint32Memory;
}

var cachedGlobalArgumentPtr = null;

function globalArgumentPtr() {
  if (cachedGlobalArgumentPtr === null) cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
  return cachedGlobalArgumentPtr;
}

function setGlobalArgument(arg, i) {
  var idx = globalArgumentPtr() / 4 + i;
  getUint32Memory()[idx] = arg;
}

var TextDecoder = (typeof self === "undefined" ? "undefined" : _typeof(self)) === 'object' && self.TextDecoder ? self.TextDecoder : __webpack_require__(/*! util */ "./node_modules/util/util.js").TextDecoder;
var cachedDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
  return cachedDecoder.decode(getUint8Memory().slice(ptr, ptr + len));
}

function getGlobalArgument(arg) {
  var idx = globalArgumentPtr() / 4 + arg;
  return getUint32Memory()[idx];
}

function evaluate(arg0) {
  var _passStringToWasm = passStringToWasm(arg0),
      _passStringToWasm2 = _slicedToArray(_passStringToWasm, 2),
      ptr0 = _passStringToWasm2[0],
      len0 = _passStringToWasm2[1];

  setGlobalArgument(len0, 0);

  try {
    var ret = wasm.evaluate(ptr0);
    var len = getGlobalArgument(0);
    var realRet = getStringFromWasm(ret, len);

    wasm.__wbindgen_free(ret, len * 1);

    return realRet;
  } finally {
    wasm.__wbindgen_free(ptr0, len0 * 1);
  }
}

var slab = [];
var slab_next = 0;

function addHeapObject(obj) {
  if (slab_next === slab.length) slab.push(slab.length + 1);
  var idx = slab_next;
  var next = slab[idx];
  slab_next = next;
  slab[idx] = {
    obj: obj,
    cnt: 1
  };
  return idx << 1;
}

var stack = [];

function getObject(idx) {
  if ((idx & 1) === 1) {
    return stack[idx >> 1];
  } else {
    var val = slab[idx >> 1];
    return val.obj;
  }
}

function __wbindgen_object_clone_ref(idx) {
  // If this object is on the stack promote it to the heap.
  if ((idx & 1) === 1) return addHeapObject(getObject(idx)); // Otherwise if the object is on the heap just bump the
  // refcount and move on

  var val = slab[idx >> 1];
  val.cnt += 1;
  return idx;
}

function dropRef(idx) {
  var obj = slab[idx >> 1];
  obj.cnt -= 1;
  if (obj.cnt > 0) return; // If we hit 0 then free up our space in the slab

  slab[idx >> 1] = slab_next;
  slab_next = idx >> 1;
}

function __wbindgen_object_drop_ref(i) {
  dropRef(i);
}

function __wbindgen_string_new(p, l) {
  return addHeapObject(getStringFromWasm(p, l));
}

function __wbindgen_number_new(i) {
  return addHeapObject(i);
}

function __wbindgen_number_get(n, invalid) {
  var obj = getObject(n);
  if (typeof obj === 'number') return obj;
  getUint8Memory()[invalid] = 1;
  return 0;
}

function __wbindgen_undefined_new() {
  return addHeapObject(undefined);
}

function __wbindgen_null_new() {
  return addHeapObject(null);
}

function __wbindgen_is_null(idx) {
  return getObject(idx) === null ? 1 : 0;
}

function __wbindgen_is_undefined(idx) {
  return getObject(idx) === undefined ? 1 : 0;
}

function __wbindgen_boolean_new(v) {
  return addHeapObject(v === 1);
}

function __wbindgen_boolean_get(i) {
  var v = getObject(i);

  if (typeof v === 'boolean') {
    return v ? 1 : 0;
  } else {
    return 2;
  }
}

function __wbindgen_symbol_new(ptr, len) {
  var a;

  if (ptr === 0) {
    a = Symbol();
  } else {
    a = Symbol(getStringFromWasm(ptr, len));
  }

  return addHeapObject(a);
}

function __wbindgen_is_symbol(i) {
  return _typeof(getObject(i)) === 'symbol' ? 1 : 0;
}

function __wbindgen_string_get(i, len_ptr) {
  var obj = getObject(i);
  if (typeof obj !== 'string') return 0;

  var _passStringToWasm3 = passStringToWasm(obj),
      _passStringToWasm4 = _slicedToArray(_passStringToWasm3, 2),
      ptr = _passStringToWasm4[0],
      len = _passStringToWasm4[1];

  getUint32Memory()[len_ptr / 4] = len;
  return ptr;
}

function __wbindgen_throw(ptr, len) {
  throw new Error(getStringFromWasm(ptr, len));
}

/***/ }),

/***/ "./src/clumsy_web_bg.wasm":
/*!********************************!*\
  !*** ./src/clumsy_web_bg.wasm ***!
  \********************************/
/*! exports provided: memory, evaluate, __wbindgen_malloc, __wbindgen_free, __wbindgen_global_argument_ptr */
/***/ (function(module, exports, __webpack_require__) {

"use strict";

// Instantiate WebAssembly module
var instance = new WebAssembly.Instance(__webpack_require__.w[module.i], {
	"./clumsy_web": {
		"__wbindgen_throw": __webpack_require__("./src/clumsy_web.js")["__wbindgen_throw"]
	}
});

// export exports from WebAssembly module
module.exports = instance.exports;

/***/ })

}]);
//# sourceMappingURL=0.bundle.js.map
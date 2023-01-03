/**
 * Minified by jsDelivr using Terser v5.15.1.
 * Original file: /npm/pagurus@0.6.1/dist/pagurus.js
 *
 * Do NOT use SRI with dynamically generated files! More information: https://www.jsdelivr.com/using-sri-with-dynamic-files
 */
/**
 * pagurus
 * Library to run Pagurus games on Web Browsers
 * @version: 0.6.0
 * @author: Takeru Ohta
 * @license: (MIT OR Apache-2.0)
 **/
!function(e,t){"object"==typeof exports&&"undefined"!=typeof module?t(exports):"function"==typeof define&&define.amd?define(["exports"],t):t((e="undefined"!=typeof globalThis?globalThis:e||self).Pagurus={})}(this,(function(e){"use strict";class t{wasmInstance;gameInstance;systemRef;memory;constructor(e,t){this.wasmInstance=e,this.gameInstance=e.exports.gameNew(),this.memory=e.exports.memory,this.systemRef=t}static async load(e){const n=new s,i={env:{systemVideoInit(e,t,s,i){n.getSystem().videoInit(e,t,s,i)},systemVideoDraw(e,t,s,i,a){n.getSystem().videoDraw(e,t,s,i,a)},systemAudioInit(e,t,s){n.getSystem().audioInit(e,t,s)},systemAudioEnqueue(e,t){n.getSystem().audioEnqueue(e,t)},systemConsoleLog(e,t){n.getSystem().consoleLog(e,t)},systemClockGameTime:()=>n.getSystem().clockGameTime(),systemClockUnixTime:()=>n.getSystem().clockUnixTime(),systemClockSetTimeout:(e,t)=>BigInt(n.getSystem().clockSetTimeout(e,t)),systemStateSave:(e,t,s,i)=>BigInt(n.getSystem().stateSave(e,t,s,i)),systemStateLoad:(e,t)=>BigInt(n.getSystem().stateLoad(e,t)),systemStateDelete:(e,t)=>BigInt(n.getSystem().stateDelete(e,t))}},a=(await WebAssembly.instantiateStreaming(fetch(e),i)).instance;return new t(a,n)}initialize(e){this.systemRef.setSystem(e);try{const e=this.wasmInstance.exports.gameInitialize(this.gameInstance);if(0!==e)throw new Error(this.getWasmString(e))}finally{this.systemRef.clearSystem()}}handleEvent(e,t){let s;this.systemRef.setSystem(e);try{t instanceof Object&&"state"in t&&"loaded"in t.state&&(s=t.state.loaded.data,t.state.loaded.data=void 0);const e=this.createWasmBytes((new TextEncoder).encode(JSON.stringify(t)));let n=0;void 0!==s&&(n=this.createWasmBytes(s));const i=this.wasmInstance.exports.gameHandleEvent(this.gameInstance,e,n);if(0===i)return!0;const a=this.getWasmString(i);if(null===JSON.parse(a))return!1;throw new Error(a)}finally{this.systemRef.clearSystem()}}query(e,t){this.systemRef.setSystem(e);try{const e=this.createWasmBytes((new TextEncoder).encode(t)),s=this.wasmInstance.exports.gameQuery(this.gameInstance,e),n=this.getWasmBytes(s);if(0===n[n.length-1])return n.subarray(0,n.length-1);{const e=new TextDecoder("utf-8").decode(n.subarray(0,n.length-1));throw new Error(e)}}finally{this.systemRef.clearSystem()}}command(e,t,s){this.systemRef.setSystem(e);try{const e=this.createWasmBytes((new TextEncoder).encode(t)),n=this.createWasmBytes(s),i=this.wasmInstance.exports.gameCommand(this.gameInstance,e,n);if(0!==i){const e=this.getWasmString(i);throw new Error(e)}}finally{this.systemRef.clearSystem()}}createWasmBytes(e){const t=this.wasmInstance.exports.memoryAllocateBytes(e.length),s=this.wasmInstance.exports.memoryBytesOffset(t),n=this.wasmInstance.exports.memoryBytesLen(t);return new Uint8Array(this.memory.buffer,s,n).set(e),t}getWasmString(e){try{const t=this.wasmInstance.exports.memoryBytesOffset(e),s=this.wasmInstance.exports.memoryBytesLen(e),n=new Uint8Array(this.memory.buffer,t,s);return new TextDecoder("utf-8").decode(n)}finally{this.wasmInstance.exports.memoryFreeBytes(e)}}getWasmBytes(e){try{const t=this.wasmInstance.exports.memoryBytesOffset(e),s=this.wasmInstance.exports.memoryBytesLen(e);return new Uint8Array(this.memory.buffer,t,s).slice()}finally{this.wasmInstance.exports.memoryFreeBytes(e)}}}class s{system;getSystem(){if(void 0===this.system)throw Error("SystemRef.system is undefined");return this.system}setSystem(e){this.system=e}clearSystem(){this.system=void 0}}function n(e){switch(e){case"Enter":return"return";case"ArrowUp":return"up";case"ArrowDown":return"down";case"ArrowLeft":return"left";case"ArrowRight":return"right";default:return}}function i(e){switch(e){case 0:return"left";case 1:return"middle";case 2:return"right";default:return}}class a{wasmMemory;db;canvas;canvasSize;audioContext;audioInputNode;audioSampleRate;startTime;nextActionId;eventQueue;resolveNextEvent;static async create(e,t={}){const s=indexedDB.open(t.databaseName||"PAGURUS_STATE_DB");return new Promise(((n,i)=>{s.onupgradeneeded=e=>{e.target.result.createObjectStore("states",{keyPath:"name"})},s.onsuccess=s=>{const i=s.target.result;n(new a(e,t.canvas,i))},s.onerror=()=>{i(new Error("failed to open database (indexedDB)"))}}))}constructor(e,t,s){this.wasmMemory=e,this.db=s,this.canvas=t,void 0!==this.canvas?this.canvasSize={width:this.canvas.width,height:this.canvas.height}:this.canvasSize={width:0,height:0},this.startTime=performance.now(),this.nextActionId=0,void 0!==this.canvas&&(document.addEventListener("keyup",(e=>{this.handleKeyup(e)&&(e.stopPropagation(),e.preventDefault())})),document.addEventListener("keydown",(e=>{this.handleKeydown(e)&&(e.stopPropagation(),e.preventDefault())})),this.canvas.addEventListener("mousemove",(e=>{this.handleMousemove(e)})),this.canvas.addEventListener("mousedown",(e=>{this.handleMousedown(e)})),this.canvas.addEventListener("mouseup",(e=>{this.handleMouseup(e)})),this.canvas.addEventListener("touchmove",(e=>{this.handleTouchmove(e),e.stopPropagation(),e.preventDefault()})),this.canvas.addEventListener("touchstart",(e=>{this.handleTouchstart(e),e.stopPropagation(),e.preventDefault()})),this.canvas.addEventListener("touchend",(e=>{this.handleTouchend(e),e.stopPropagation(),e.preventDefault()})));const n={window:{redrawNeeded:{size:this.canvasSize}}};this.eventQueue=[n]}nextEvent(){const e=this.eventQueue.shift();return void 0!==e?Promise.resolve(e):new Promise((e=>{this.resolveNextEvent=e}))}handleKeyup(e){const t=n(e.key);return void 0!==t&&this.enqueueEvent({key:{up:{key:t}}}),void 0!==t}handleKeydown(e){const t=n(e.key);return void 0!==t&&this.enqueueEvent({key:{down:{key:t}}}),void 0!==t}touchPosition(e){if(void 0===this.canvas)throw new Error("bug");const t=this.canvas.getBoundingClientRect();return{x:Math.round(e.clientX-t.left),y:Math.round(e.clientY-t.top)}}handleTouchmove(e){const t=e.changedTouches;for(let e=0;e<t.length;e++){const s=t[e],n=this.touchPosition(s);this.enqueueEvent({mouse:{move:{position:n}}});break}}handleTouchstart(e){const t=e.changedTouches;for(let e=0;e<t.length;e++){const s=t[e],n="left",i=this.touchPosition(s);this.enqueueEvent({mouse:{down:{position:i,button:n}}});break}}handleTouchend(e){const t=e.changedTouches;for(let e=0;e<t.length;e++){const s=t[e],n="left",i=this.touchPosition(s);this.enqueueEvent({mouse:{up:{position:i,button:n}}});break}}handleMousemove(e){const t=e.offsetX,s=e.offsetY;this.enqueueEvent({mouse:{move:{position:{x:t,y:s}}}})}handleMousedown(e){const t=e.offsetX,s=e.offsetY,n=i(e.button);void 0!==n&&this.enqueueEvent({mouse:{down:{position:{x:t,y:s},button:n}}})}handleMouseup(e){const t=e.offsetX,s=e.offsetY,n=i(e.button);void 0!==n&&this.enqueueEvent({mouse:{up:{position:{x:t,y:s},button:n}}})}enqueueEvent(e){void 0!==this.resolveNextEvent?(this.resolveNextEvent(e),this.resolveNextEvent=void 0):this.eventQueue.push(e)}notifyRedrawNeeded(){void 0!==this.canvas&&(this.canvasSize={width:this.canvas.width,height:this.canvas.height},this.enqueueEvent({window:{redrawNeeded:{size:this.canvasSize}}}))}videoInit(e,t,s,n){new DataView(this.wasmMemory.buffer).setUint8(s,1),new DataView(this.wasmMemory.buffer).setUint32(n,e,!0)}videoDraw(e,t,s,n,i){if(void 0===this.canvas)return;if(1!=i)throw new Error(`expected RGB32(3) format, but got ${i}`);if(s!=n)throw new Error(`width ${s} differs from stride ${n}`);if(this.canvasSize.width!=this.canvas.width||this.canvasSize.height!=this.canvas.height)return this.canvasSize={width:this.canvas.width,height:this.canvas.height},void this.enqueueEvent({window:{redrawNeeded:{size:this.canvasSize}}});if(0===s||0===t)return;const a=this.canvas.getContext("2d");if(!a)throw Error("failed to get canvas 2D context");const o=t/4/s,r=new Uint8ClampedArray(this.wasmMemory.buffer,e,t);if(s==this.canvas.width&&o==this.canvas.height){const e=new ImageData(r,s,o);a.putImageData(e,0,0)}else{const e=new ImageData(r.slice(),s,o);createImageBitmap(e).then((e=>{if(void 0===this.canvas)throw new Error("bug");a.drawImage(e,0,0,this.canvas.width,this.canvas.height)})).catch((e=>{throw e}))}}audioInit(e,t,s){var n;this.audioSampleRate=e,(n=new ArrayBuffer(2),new DataView(n).setInt16(0,256,!0),256===new Int16Array(n)[0])?new DataView(this.wasmMemory.buffer).setUint8(s,3):new DataView(this.wasmMemory.buffer).setUint8(s,2)}audioEnqueue(e,t){if(void 0===this.audioSampleRate)return void console.warn("audioInit() has not been called yet");const s=new Float32Array(this.wasmMemory.buffer,e,t/4).slice();if(void 0===this.audioContext){const e=new Blob(['\nclass PagurusAudioWorkletProcessor extends AudioWorkletProcessor {\n  constructor() {\n    super();\n    this.inputBuffer = [];\n    this.offset = 0;\n    this.port.onmessage = (e) => {\n      this.inputBuffer.push(e.data);\n    };\n  }\n\n  process(inputs, outputs, parameters) {\n    const outputChannel = outputs[0][0];\n    for (let i = 0; i < outputChannel.length; i++) {\n      const audioData = this.inputBuffer[0];\n      if (audioData === undefined) {\n        outputChannel[i] = 0;\n      } else {\n        outputChannel[i] = audioData[this.offset];\n        this.offset++;\n        if (this.offset == audioData.length) {\n          this.inputBuffer.shift();\n          this.offset = 0;\n        }\n      }\n    }\n    return true;\n  }\n}\n\nregisterProcessor("pagurus-audio-worklet-processor", PagurusAudioWorkletProcessor);\n'],{type:"application/javascript"}),t=new AudioContext({sampleRate:this.audioSampleRate});this.audioContext=t,this.audioContext.audioWorklet.addModule(URL.createObjectURL(e)).then((()=>{this.audioInputNode=new AudioWorkletNode(t,"pagurus-audio-worklet-processor"),this.audioInputNode.connect(t.destination),this.audioInputNode.port.postMessage(s,[s.buffer])})).catch((e=>{throw e}))}else void 0!==this.audioInputNode&&this.audioInputNode.port.postMessage(s,[s.buffer])}consoleLog(e,t){const s=this.getWasmString(e,t);console.log(s)}clockGameTime(){return(performance.now()-this.startTime)/1e3}clockUnixTime(){return(new Date).getTime()/1e3}clockSetTimeout(e,t){const s=this.getNextActionId();return setTimeout((()=>{this.enqueueEvent({timeout:{id:s,tag:e}})}),1e3*t),s}stateSave(e,t,s,n){const i=this.getNextActionId(),a=this.getWasmString(e,t),o=new Uint8Array(this.wasmMemory.buffer,s,n).slice(),r=this.db.transaction(["states"],"readwrite").objectStore("states").put({name:a,data:o});return r.onsuccess=()=>{this.enqueueEvent({state:{saved:{id:i}}})},r.onerror=()=>{this.enqueueEvent({state:{saved:{id:i,failed:{message:"PUT_FAILURE"}}}})},i}stateLoad(e,t){const s=this.getNextActionId(),n=this.getWasmString(e,t),i=this.db.transaction(["states"],"readwrite").objectStore("states").get(n);return i.onsuccess=e=>{if(void 0===e.target.result)this.enqueueEvent({state:{loaded:{id:s}}});else{const t=e.target.result.data;this.enqueueEvent({state:{loaded:{id:s,data:t}}})}},i.onerror=()=>{this.enqueueEvent({state:{loaded:{id:s,failed:{message:"GET_FAILURE"}}}})},s}stateDelete(e,t){const s=this.getNextActionId(),n=this.getWasmString(e,t),i=this.db.transaction(["states"],"readwrite").objectStore("states").delete(n);return i.onsuccess=()=>{this.enqueueEvent({state:{deleted:{id:s}}})},i.onerror=()=>{this.enqueueEvent({state:{deleted:{id:s,failed:{message:"DELETE_FAILURE"}}}})},s}getWasmString(e,t){const s=new Uint8Array(this.wasmMemory.buffer,e,t);return new TextDecoder("utf-8").decode(s)}getNextActionId(){const e=this.nextActionId;return this.nextActionId=this.nextActionId+1,e}}e.Game=t,e.System=a}));
//# sourceMappingURL=/sm/13f7d9650e26f0971e7785d13239aa2d98a1c51b8de2fd5c3501317e2b0a7f56.map
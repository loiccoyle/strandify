(()=>{"use strict";var t,n,e,r,o={237:(t,n,e)=>{e.a(t,(async(t,n)=>{try{e(804);var r=e(538),o=t([r]);r=(o.then?(await o)():o)[0];const i=1080,a=1080,s={SINGLE:"single",LINE:"line",BOX:"box",CIRCLE:"circle",ERASER:"eraser"};let c={image:null,imageData:null,pegs:[],isDragging:!1,draggedPegIndex:-1,isDrawing:!1,isErasing:!1,startX:0,startY:0};const u=document.getElementById("canvas"),_=u.getContext("2d"),l=document.getElementById("imageUpload"),d=document.getElementById("brushType"),g=document.getElementById("pegCount"),f=document.getElementById("clearBtn"),p=document.getElementById("run");function b(t){const n=new Blob([t],{type:"image/svg+xml;charset=utf-8"}),e=URL.createObjectURL(n),r=document.createElement("a");r.href=e,r.download="strandify.svg",document.body.appendChild(r),r.click(),document.body.removeChild(r)}function h(t){const n=/^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(t);return n?[parseInt(n[1],16),parseInt(n[2],16),parseInt(n[3],16)]:null}async function y(){document.addEventListener("mousemove",x),l.addEventListener("change",w),u.addEventListener("mousedown",v),f.addEventListener("click",F),document.addEventListener("mouseup",I)}function w(t){F();const n=t.target.files[0],e=new FileReader;e.onload=t=>{c.image=new Image,c.image.onload=()=>{let{width:t,height:n}=c.image;const e=t/n;(t>i||n>a)&&(e>1?(t=i,n=t/e):(n=a,t=n*e)),u.width=t,u.height=n,m(),u.toBlob((t=>t.arrayBuffer().then((t=>{c.imageData=new Uint8Array(t)}))))},c.image.src=t.target.result},e.readAsDataURL(n),p.disabled=!1}function m(){_.clearRect(0,0,u.width,u.height),c.image&&_.drawImage(c.image,0,0,u.width,u.height),c.pegs.forEach((t=>{_.beginPath(),_.arc(t.x,t.y,5,0,2*Math.PI),_.fillStyle="red",_.fill()}))}function v(t){const n=u.getBoundingClientRect();if(c.startX=Math.max(t.clientX-n.left,0),c.startY=Math.max(t.clientY-n.top,0),d.value!==s.ERASER){const t=(e=c.startX,r=c.startY,c.pegs.findIndex((t=>Math.hypot(e-t.x,r-t.y)<5)));if(-1!==t)return c.isDragging=!0,void(c.draggedPegIndex=t)}var e,r;d.value===s.SINGLE?(c.pegs.push({x:c.startX,y:c.startY}),m()):d.value===s.ERASER?c.isErasing=!0:c.isDrawing=!0}function x(t){const{x:n,y:e}=E(t);var r,o,i,a;k(n,e),c.isDragging&&-1!==c.draggedPegIndex?(c.pegs[c.draggedPegIndex]={x:n,y:e},m()):c.isDrawing?(m(),S(c.startX,c.startY,n,e)):c.isErasing&&(m(),r=c.startX,o=c.startY,i=n,a=e,_.strokeStyle="rgba(255, 0, 0, 0.5)",_.strokeRect(Math.min(r,i),Math.min(o,a),Math.abs(i-r),Math.abs(a-o)))}function I(t){const{x:n,y:e}=E(t);c.isDrawing?C(c.startX,c.startY,n,e):c.isErasing&&B(c.startX,c.startY,n,e),c.isDragging=!1,c.isDrawing=!1,c.isErasing=!1,c.draggedPegIndex=-1,m()}function E(t){const n=u.getBoundingClientRect();return{x:Math.min(Math.max(t.clientX-n.left,0),u.width-1),y:Math.min(Math.max(t.clientY-n.top,0),u.height-1)}}function k(t,n){u.style.cursor=M(t,n)?"pointer":"crosshair"}function M(t,n){return c.pegs.some((e=>Math.hypot(t-e.x,n-e.y)<5))}function S(t,n,e,r){const o=R(t,n,e,r);_.strokeStyle="rgba(255, 0, 0, 0.5)",_.fillStyle="rgba(255, 0, 0, 0.5)",o.forEach((t=>{_.beginPath(),_.arc(t.x,t.y,5,0,2*Math.PI),_.fill()})),_.beginPath(),_.moveTo(o[0].x,o[0].y),o.slice(1).forEach((t=>_.lineTo(t.x,t.y))),_.closePath(),_.stroke()}function R(t,n,e,o){const i=parseInt(g.value);if(isNaN(i)||i<2)return[];let a;switch(d.value){case s.LINE:a=(0,r.br)(t,n,e,o,i);break;case s.BOX:a=(0,r.SU)(Math.min(t,e),Math.min(n,o),Math.abs(t-e),Math.abs(n-o),i);break;case s.CIRCLE:const c=Math.hypot(e-t,o-n);a=(0,r.Z5)(t,n,c,i);break;default:return[]}const c=a.get_x(),_=a.get_y();let l=[];return c.forEach(((t,n)=>l.push({x:Math.min(t,u.width-1),y:Math.min(_[n],u.height-1)}))),l}function C(t,n,e,r){c.pegs=c.pegs.concat(R(t,n,e,r))}function B(t,n,e,r){Math.hypot(t-e,n-r)<5?T(t,n):A(Math.min(t,e),Math.min(n,r),Math.max(t,e),Math.max(n,r))}function T(t,n){const e=c.pegs.findIndex((e=>Math.hypot(t-e.x,n-e.y)<5));-1!==e&&c.pegs.splice(e,1)}function A(t,n,e,r){c.pegs=c.pegs.filter((o=>!(o.x>=t&&o.x<=e&&o.y>=n&&o.y<=r)))}function F(){c.pegs=[],m()}document.getElementById("removeImage").addEventListener("click",(function(){c.image=null,document.getElementById("imageUpload").value="",u.width=600,u.height=600,m(),p.disabled=!0})),p.addEventListener("click",(function(){if(0==c.pegs.length)return void alert("Please add some pegs first");const t={iterations:parseInt(document.getElementById("iterations").value),patherYarn:{width:parseFloat(document.getElementById("patherYarnWidth").value),opacity:parseFloat(document.getElementById("patherYarnOpacity").value)},yarn:{width:parseFloat(document.getElementById("yarnWidth").value),opacity:parseFloat(document.getElementById("yarnOpacity").value),color:h(document.getElementById("yarnColor").value)},early_stop:{loss_threshold:document.getElementById("lossThreshold").value?parseFloat(document.getElementById("lossThreshold").value):null,max_count:parseInt(document.getElementById("maxCount").value)},start_peg_radius:10,skip_peg_within:parseInt(document.getElementById("skipPegWithin").value),beam_width:parseInt(document.getElementById("beamWidth").value)};console.log("config:",t);let n=new r.az(t.early_stop.loss_threshold,t.early_stop.max_count),e=new r.RX(t.yarn.width,t.yarn.opacity,t.yarn.color[0],t.yarn.color[1],t.yarn.color[2]),o=new r.TQ(t.iterations,new r.RX(t.patherYarn.width,t.patherYarn.opacity,0,0,0),n,t.start_peg_radius,t.skip_peg_within,t.beam_width),i=c.pegs.map((t=>new r.b4(t.x,t.y)));console.log(c);const a=(0,r.qq)(c.imageData,i,o,e),s=document.getElementById("svg-container");s.innerHTML=a;const u=document.createElement("button");u.id="downloadBtn",u.innerHTML="Download SVG",u.addEventListener("click",(function(){b(a)})),s.appendChild(u),s.style.display="flex"})),y(),n()}catch(j){n(j)}}))},83:(t,n,e)=>{e.d(n,{A:()=>s});var r=e(601),o=e.n(r),i=e(314),a=e.n(i)()(o());a.push([t.id,'body {\n  font-family: Arial, sans-serif;\n  display: flex;\n  flex-direction: column;\n  align-items: center;\n  width: 100%;\n  margin: 0;\n  color: lightgray;\n  background-color: #121212;\n}\nh1 {\n  margin: 0;\n}\na {\n  color: white;\n  text-decoration: none;\n  font-weight: bold;\n}\nbutton {\n  background-color: #333333;\n  color: white;\n  border: none;\n  border-radius: 5px;\n  cursor: pointer;\n  margin: 0 5px;\n  padding: 5px 10px;\n}\nbutton:disabled {\n  color: gray;\n  cursor: not-allowed;\n}\n\n.header {\n  padding: 10px;\n  margin-bottom: 20px;\n  width: 100%;\n  background-color: #222222;\n  display: flex;\n  align-items: center;\n  justify-content: space-between;\n  box-sizing: border-box;\n}\n.links {\n  display: flex;\n  gap: 20px;\n}\n.columns {\n  display: flex;\n  flex-direction: row;\n  gap: 20px;\n  flex-wrap: wrap;\n}\ncanvas {\n  border: 1px solid lightgray;\n  background-color: black;\n  margin-top: 20px;\n}\n.controls {\n  margin-top: 10px;\n  display: flex;\n  gap: 10px;\n  align-items: center;\n  flex-wrap: wrap;\n  justify-content: center;\n}\n\nselect,\ninput[type="number"],\ninput[type="color"],\ninput[type="file"] {\n  margin: 0 5px;\n  color: lightgray;\n  background-color: #333333;\n  border: none;\n  border-radius: 5px;\n  padding: 5px;\n}\ninput[type="number"],\ninput[type="color"] {\n  width: 100px;\n}\n.config-container {\n  display: flex;\n  flex-direction: column;\n  margin-top: 20px;\n  gap: 10px;\n}\n.config-section {\n  padding: 10px;\n  border: 1px solid #ccc;\n  border-radius: 5px;\n  background-color: #222222;\n}\n.config-section h2 {\n  margin-top: 0;\n}\n.form-group {\n  margin-bottom: 10px;\n  display: flex;\n  justify-content: space-between;\n}\nlabel {\n  display: inline-block;\n}\n.image-upload {\n  margin-bottom: 10px;\n  display: flex;\n  justify-content: center;\n}\n\n#svg-container {\n  background-color: #222222;\n  border-radius: 5px;\n  border: 1px solid lightgray;\n  overflow: auto;\n  padding: 20px;\n  flex-direction: column;\n  gap: 10px;\n  align-items: center;\n  margin-top: 20px;\n  display: none;\n}\n',""]);const s=a},314:t=>{t.exports=function(t){var n=[];return n.toString=function(){return this.map((function(n){var e="",r=void 0!==n[5];return n[4]&&(e+="@supports (".concat(n[4],") {")),n[2]&&(e+="@media ".concat(n[2]," {")),r&&(e+="@layer".concat(n[5].length>0?" ".concat(n[5]):""," {")),e+=t(n),r&&(e+="}"),n[2]&&(e+="}"),n[4]&&(e+="}"),e})).join("")},n.i=function(t,e,r,o,i){"string"==typeof t&&(t=[[null,t,void 0]]);var a={};if(r)for(var s=0;s<this.length;s++){var c=this[s][0];null!=c&&(a[c]=!0)}for(var u=0;u<t.length;u++){var _=[].concat(t[u]);r&&a[_[0]]||(void 0!==i&&(void 0===_[5]||(_[1]="@layer".concat(_[5].length>0?" ".concat(_[5]):""," {").concat(_[1],"}")),_[5]=i),e&&(_[2]?(_[1]="@media ".concat(_[2]," {").concat(_[1],"}"),_[2]=e):_[2]=e),o&&(_[4]?(_[1]="@supports (".concat(_[4],") {").concat(_[1],"}"),_[4]=o):_[4]="".concat(o)),n.push(_))}},n}},601:t=>{t.exports=function(t){return t[1]}},804:(t,n,e)=>{var r=e(72),o=e.n(r),i=e(825),a=e.n(i),s=e(659),c=e.n(s),u=e(56),_=e.n(u),l=e(540),d=e.n(l),g=e(113),f=e.n(g),p=e(83),b={};b.styleTagTransform=f(),b.setAttributes=_(),b.insert=c().bind(null,"head"),b.domAPI=a(),b.insertStyleElement=d(),o()(p.A,b),p.A&&p.A.locals&&p.A.locals},72:t=>{var n=[];function e(t){for(var e=-1,r=0;r<n.length;r++)if(n[r].identifier===t){e=r;break}return e}function r(t,r){for(var i={},a=[],s=0;s<t.length;s++){var c=t[s],u=r.base?c[0]+r.base:c[0],_=i[u]||0,l="".concat(u," ").concat(_);i[u]=_+1;var d=e(l),g={css:c[1],media:c[2],sourceMap:c[3],supports:c[4],layer:c[5]};if(-1!==d)n[d].references++,n[d].updater(g);else{var f=o(g,r);r.byIndex=s,n.splice(s,0,{identifier:l,updater:f,references:1})}a.push(l)}return a}function o(t,n){var e=n.domAPI(n);return e.update(t),function(n){if(n){if(n.css===t.css&&n.media===t.media&&n.sourceMap===t.sourceMap&&n.supports===t.supports&&n.layer===t.layer)return;e.update(t=n)}else e.remove()}}t.exports=function(t,o){var i=r(t=t||[],o=o||{});return function(t){t=t||[];for(var a=0;a<i.length;a++){var s=e(i[a]);n[s].references--}for(var c=r(t,o),u=0;u<i.length;u++){var _=e(i[u]);0===n[_].references&&(n[_].updater(),n.splice(_,1))}i=c}}},659:t=>{var n={};t.exports=function(t,e){var r=function(t){if(void 0===n[t]){var e=document.querySelector(t);if(window.HTMLIFrameElement&&e instanceof window.HTMLIFrameElement)try{e=e.contentDocument.head}catch(t){e=null}n[t]=e}return n[t]}(t);if(!r)throw new Error("Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.");r.appendChild(e)}},540:t=>{t.exports=function(t){var n=document.createElement("style");return t.setAttributes(n,t.attributes),t.insert(n,t.options),n}},56:(t,n,e)=>{t.exports=function(t){var n=e.nc;n&&t.setAttribute("nonce",n)}},825:t=>{t.exports=function(t){if("undefined"==typeof document)return{update:function(){},remove:function(){}};var n=t.insertStyleElement(t);return{update:function(e){!function(t,n,e){var r="";e.supports&&(r+="@supports (".concat(e.supports,") {")),e.media&&(r+="@media ".concat(e.media," {"));var o=void 0!==e.layer;o&&(r+="@layer".concat(e.layer.length>0?" ".concat(e.layer):""," {")),r+=e.css,o&&(r+="}"),e.media&&(r+="}"),e.supports&&(r+="}");var i=e.sourceMap;i&&"undefined"!=typeof btoa&&(r+="\n/*# sourceMappingURL=data:application/json;base64,".concat(btoa(unescape(encodeURIComponent(JSON.stringify(i))))," */")),n.styleTagTransform(r,t,n.options)}(n,t,e)},remove:function(){!function(t){if(null===t.parentNode)return!1;t.parentNode.removeChild(t)}(n)}}}},113:t=>{t.exports=function(t,n){if(n.styleSheet)n.styleSheet.cssText=t;else{for(;n.firstChild;)n.removeChild(n.firstChild);n.appendChild(document.createTextNode(t))}}},538:(t,n,e)=>{e.a(t,(async(t,r)=>{try{e.d(n,{RX:()=>i.RX,SU:()=>i.SU,TQ:()=>i.TQ,Z5:()=>i.Z5,az:()=>i.az,b4:()=>i.b4,br:()=>i.br,qq:()=>i.qq});var o=e(223),i=e(246),a=t([o]);o=(a.then?(await a)():a)[0],(0,i.lI)(o),o.__wbindgen_start(),r()}catch(t){r(t)}}))},246:(t,n,e)=>{let r;function o(t){r=t}e.d(n,{BZ:()=>W,D1:()=>H,DI:()=>ft,FJ:()=>mt,GB:()=>rt,Gu:()=>Z,J1:()=>dt,KN:()=>it,Mq:()=>wt,NL:()=>Q,O9:()=>at,OL:()=>et,PR:()=>tt,Py:()=>gt,Qn:()=>vt,RX:()=>q,Rv:()=>ot,SU:()=>k,TQ:()=>F,V5:()=>X,VF:()=>yt,Xu:()=>Y,Z5:()=>I,az:()=>T,b4:()=>L,bk:()=>$,br:()=>E,cA:()=>G,cl:()=>nt,en:()=>N,hH:()=>pt,hW:()=>bt,h_:()=>J,jF:()=>ct,lI:()=>o,nq:()=>ht,qq:()=>R,qv:()=>V,rl:()=>xt,s:()=>K,tM:()=>lt,tn:()=>st,u$:()=>z,vU:()=>ut,xN:()=>_t,yc:()=>U});let i=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=null;function s(){return null!==a&&0!==a.byteLength||(a=new Uint8Array(r.memory.buffer)),a}function c(t,n){return t>>>=0,i.decode(s().subarray(t,t+n))}const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);let _=u.length;function l(t){_===u.length&&u.push(u.length+1);const n=_;return _=u[n],u[n]=t,n}function d(t){return u[t]}function g(t){const n=d(t);return function(t){t<132||(u[t]=_,_=t)}(t),n}function f(t){const n=typeof t;if("number"==n||"boolean"==n||null==t)return`${t}`;if("string"==n)return`"${t}"`;if("symbol"==n){const n=t.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==n){const n=t.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(t)){const n=t.length;let e="[";n>0&&(e+=f(t[0]));for(let r=1;r<n;r++)e+=", "+f(t[r]);return e+="]",e}const e=/\[object ([^\]]+)\]/.exec(toString.call(t));let r;if(!(e.length>1))return toString.call(t);if(r=e[1],"Object"==r)try{return"Object("+JSON.stringify(t)+")"}catch(t){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:r}let p=0,b=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof b.encodeInto?function(t,n){return b.encodeInto(t,n)}:function(t,n){const e=b.encode(t);return n.set(e),{read:t.length,written:e.length}};function y(t,n,e){if(void 0===e){const e=b.encode(t),r=n(e.length,1)>>>0;return s().subarray(r,r+e.length).set(e),p=e.length,r}let r=t.length,o=n(r,1)>>>0;const i=s();let a=0;for(;a<r;a++){const n=t.charCodeAt(a);if(n>127)break;i[o+a]=n}if(a!==r){0!==a&&(t=t.slice(a)),o=e(o,r,r=a+3*t.length,1)>>>0;const n=s().subarray(o+a,o+r);a+=h(t,n).written,o=e(o,r,a,1)>>>0}return p=a,o}let w=null;function m(){return(null===w||!0===w.buffer.detached||void 0===w.buffer.detached&&w.buffer!==r.memory.buffer)&&(w=new DataView(r.memory.buffer)),w}let v=null;function x(t,n){return t>>>=0,(null!==v&&0!==v.byteLength||(v=new Uint32Array(r.memory.buffer)),v).subarray(t/4,t/4+n)}function I(t,n,e,o){const i=r.circleCoords(t,n,e,o);return O.__wrap(i)}function E(t,n,e,o,i){const a=r.lineCoords(t,n,e,o,i);return O.__wrap(a)}function k(t,n,e,o,i){const a=r.rectangleCoords(t,n,e,o,i);return O.__wrap(a)}function M(t){return null==t}function S(t,n){if(!(t instanceof n))throw new Error(`expected instance of ${n.name}`);return t.ptr}function R(t,n,e,o){let i,a;try{const v=r.__wbindgen_add_to_stack_pointer(-16),x=function(t,n){const e=n(1*t.length,1)>>>0;return s().set(t,e/1),p=t.length,e}(t,r.__wbindgen_export_0),I=p,E=function(t,n){const e=n(4*t.length,4)>>>0,r=m();for(let n=0;n<t.length;n++)r.setUint32(e+4*n,l(t[n]),!0);return p=t.length,e}(n,r.__wbindgen_export_0),k=p;S(e,F);var u=e.__destroy_into_raw();S(o,q);var _=o.__destroy_into_raw();r.computeSvg(v,x,I,E,k,u,_);var d=m().getInt32(v+0,!0),f=m().getInt32(v+4,!0),b=m().getInt32(v+8,!0),h=m().getInt32(v+12,!0),y=d,w=f;if(h)throw y=0,w=0,g(b);return i=y,a=w,c(y,w)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_2(i,a,1)}}function C(t,n){try{return t.apply(this,n)}catch(t){r.__wbindgen_export_3(l(t))}}Object.freeze({Cs420:0,0:"Cs420",Cs422:1,1:"Cs422",Cs444:2,2:"Cs444",Cs400:3,3:"Cs400"});const B="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((t=>r.__wbg_earlystopconfig_free(t>>>0,1)));class T{__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,B.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_earlystopconfig_free(t,0)}constructor(t,n){const e=r.earlystopconfig_new(!M(t),M(t)?0:t,n);return this.__wbg_ptr=e>>>0,B.register(this,this.__wbg_ptr,this),this}}const A="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((t=>r.__wbg_patherconfig_free(t>>>0,1)));class F{__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,A.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_patherconfig_free(t,0)}constructor(t,n,e,o,i,a){S(n,q);var s=n.__destroy_into_raw();S(e,T);var c=e.__destroy_into_raw();const u=r.patherconfig_new(t,s,c,o,i,a);return this.__wbg_ptr=u>>>0,A.register(this,this.__wbg_ptr,this),this}}const j="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((t=>r.__wbg_peg_free(t>>>0,1)));class L{static __wrap(t){t>>>=0;const n=Object.create(L.prototype);return n.__wbg_ptr=t,j.register(n,n.__wbg_ptr,n),n}static __unwrap(t){return t instanceof L?t.__destroy_into_raw():0}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,j.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_peg_free(t,0)}constructor(t,n){const e=r.peg_new(t,n);return this.__wbg_ptr=e>>>0,j.register(this,this.__wbg_ptr,this),this}withJitter(t){const n=r.peg_withJitter(this.__wbg_ptr,t);return L.__wrap(n)}}const P="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((t=>r.__wbg_shapecoords_free(t>>>0,1)));class O{static __wrap(t){t>>>=0;const n=Object.create(O.prototype);return n.__wbg_ptr=t,P.register(n,n.__wbg_ptr,n),n}__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,P.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_shapecoords_free(t,0)}get_x(){try{const o=r.__wbindgen_add_to_stack_pointer(-16);r.shapecoords_get_x(o,this.__wbg_ptr);var t=m().getInt32(o+0,!0),n=m().getInt32(o+4,!0),e=x(t,n).slice();return r.__wbindgen_export_2(t,4*n,4),e}finally{r.__wbindgen_add_to_stack_pointer(16)}}get_y(){try{const o=r.__wbindgen_add_to_stack_pointer(-16);r.shapecoords_get_y(o,this.__wbg_ptr);var t=m().getInt32(o+0,!0),n=m().getInt32(o+4,!0),e=x(t,n).slice();return r.__wbindgen_export_2(t,4*n,4),e}finally{r.__wbindgen_add_to_stack_pointer(16)}}}const D="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((t=>r.__wbg_yarn_free(t>>>0,1)));class q{__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,D.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_yarn_free(t,0)}constructor(t,n,e,o,i){const a=r.yarn_new(t,n,e,o,i);return this.__wbg_ptr=a>>>0,D.register(this,this.__wbg_ptr,this),this}}function U(t,n){return l(c(t,n))}function N(t){return L.__unwrap(g(t))}function X(){return l(new Error)}function z(t,n){const e=y(d(n).stack,r.__wbindgen_export_0,r.__wbindgen_export_1),o=p;m().setInt32(t+4,o,!0),m().setInt32(t+0,e,!0)}function Y(t,n){let e,o;try{e=t,o=n,console.error(c(t,n))}finally{r.__wbindgen_export_2(e,o,1)}}function $(t){g(t)}function W(t){return l(d(t))}function J(t){return l(d(t).crypto)}function V(t){const n=d(t);return"object"==typeof n&&null!==n}function G(t){return l(d(t).process)}function H(t){return l(d(t).versions)}function Q(t){return l(d(t).node)}function Z(t){return"string"==typeof d(t)}function K(){return C((function(){return l(module.require)}),arguments)}function tt(t){return"function"==typeof d(t)}function nt(t){return l(d(t).msCrypto)}function et(t){return l(new Uint8Array(t>>>0))}function rt(){return C((function(t,n){return l(Reflect.get(d(t),d(n)))}),arguments)}function ot(t){return d(t).now()}function it(){return C((function(){return l(self.self)}),arguments)}function at(){return C((function(){return l(window.window)}),arguments)}function st(){return C((function(){return l(globalThis.globalThis)}),arguments)}function ct(){return C((function(){return l(global.global)}),arguments)}function ut(t){return void 0===d(t)}function _t(t,n){return l(new Function(c(t,n)))}function lt(){return C((function(t,n){return l(d(t).call(d(n)))}),arguments)}function dt(){return C((function(t,n,e){return l(d(t).call(d(n),d(e)))}),arguments)}function gt(){return l(r.memory)}function ft(t){return l(d(t).buffer)}function pt(t,n,e){return l(new Uint8Array(d(t),n>>>0,e>>>0))}function bt(){return C((function(t,n){d(t).randomFillSync(g(n))}),arguments)}function ht(t,n,e){return l(d(t).subarray(n>>>0,e>>>0))}function yt(){return C((function(t,n){d(t).getRandomValues(d(n))}),arguments)}function wt(t){return l(new Uint8Array(d(t)))}function mt(t,n,e){d(t).set(d(n),e>>>0)}function vt(t,n){throw new Error(c(t,n))}function xt(t,n){const e=y(f(d(n)),r.__wbindgen_export_0,r.__wbindgen_export_1),o=p;m().setInt32(t+4,o,!0),m().setInt32(t+0,e,!0)}},223:(t,n,e)=>{var r=e(246);t.exports=e.v(n,t.id,"bb1189075da0446905da",{"./strandify_wasm_bg.js":{__wbindgen_string_new:r.yc,__wbg_peg_unwrap:r.en,__wbg_new_abda76e883ba8a5f:r.V5,__wbg_stack_658279fe44541cf6:r.u$,__wbg_error_f851667af71bcfc6:r.Xu,__wbindgen_object_drop_ref:r.bk,__wbindgen_object_clone_ref:r.BZ,__wbg_crypto_1d1f22824a6a080c:r.h_,__wbindgen_is_object:r.qv,__wbg_process_4a72847cc503995b:r.cA,__wbg_versions_f686565e586dd935:r.D1,__wbg_node_104a2ff8d6ea03a2:r.NL,__wbindgen_is_string:r.Gu,__wbg_require_cca90b1a94a0255b:r.s,__wbindgen_is_function:r.PR,__wbg_msCrypto_eb05e62b530a1508:r.cl,__wbg_newwithlength_ec548f448387c968:r.OL,__wbg_get_224d16597dbbfd96:r.GB,__wbg_now_a69647afb1f66247:r.Rv,__wbg_self_3093d5d1f7bcb682:r.KN,__wbg_window_3bcfc4d31bc012f8:r.O9,__wbg_globalThis_86b222e13bdf32ed:r.tn,__wbg_global_e5a3fe56f8be9485:r.jF,__wbindgen_is_undefined:r.vU,__wbg_newnoargs_76313bd6ff35d0f2:r.xN,__wbg_call_1084a111329e68ce:r.tM,__wbg_call_89af060b4e1523f2:r.J1,__wbindgen_memory:r.Py,__wbg_buffer_b7b08af79b0b0974:r.DI,__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9:r.hH,__wbg_randomFillSync_5c9c955aa56b6049:r.hW,__wbg_subarray_7c2e3576afe181d1:r.nq,__wbg_getRandomValues_3aa56aa6edec874c:r.VF,__wbg_new_ea1883e1e5e86686:r.Mq,__wbg_set_d1e79e2388520f18:r.FJ,__wbindgen_throw:r.Qn,__wbindgen_debug_string:r.rl}})}},i={};function a(t){var n=i[t];if(void 0!==n)return n.exports;var e=i[t]={id:t,exports:{}};return o[t](e,e.exports,a),e.exports}t="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",n="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",e="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",r=t=>{t&&t.d<1&&(t.d=1,t.forEach((t=>t.r--)),t.forEach((t=>t.r--?t.r++:t())))},a.a=(o,i,a)=>{var s;a&&((s=[]).d=-1);var c,u,_,l=new Set,d=o.exports,g=new Promise(((t,n)=>{_=n,u=t}));g[n]=d,g[t]=t=>(s&&t(s),l.forEach(t),g.catch((t=>{}))),o.exports=g,i((o=>{var i;c=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[t])return o;if(o.then){var i=[];i.d=0,o.then((t=>{a[n]=t,r(i)}),(t=>{a[e]=t,r(i)}));var a={};return a[t]=t=>t(i),a}}var s={};return s[t]=t=>{},s[n]=o,s})))(o);var a=()=>c.map((t=>{if(t[e])throw t[e];return t[n]})),u=new Promise((n=>{(i=()=>n(a)).r=0;var e=t=>t!==s&&!l.has(t)&&(l.add(t),t&&!t.d&&(i.r++,t.push(i)));c.map((n=>n[t](e)))}));return i.r?u:a()}),(t=>(t?_(g[e]=t):u(d),r(s)))),s&&s.d<0&&(s.d=0)},a.n=t=>{var n=t&&t.__esModule?()=>t.default:()=>t;return a.d(n,{a:n}),n},a.d=(t,n)=>{for(var e in n)a.o(n,e)&&!a.o(t,e)&&Object.defineProperty(t,e,{enumerable:!0,get:n[e]})},a.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(t){if("object"==typeof window)return window}}(),a.o=(t,n)=>Object.prototype.hasOwnProperty.call(t,n),a.v=(t,n,e,r)=>{var o=fetch(a.p+""+e+".module.wasm"),i=()=>o.then((t=>t.arrayBuffer())).then((t=>WebAssembly.instantiate(t,r))).then((n=>Object.assign(t,n.instance.exports)));return o.then((n=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,r).then((n=>Object.assign(t,n.instance.exports)),(t=>{if("application/wasm"!==n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",t),i();throw t})):i()))},(()=>{var t;a.g.importScripts&&(t=a.g.location+"");var n=a.g.document;if(!t&&n&&(n.currentScript&&"SCRIPT"===n.currentScript.tagName.toUpperCase()&&(t=n.currentScript.src),!t)){var e=n.getElementsByTagName("script");if(e.length)for(var r=e.length-1;r>-1&&(!t||!/^http(s?):/.test(t));)t=e[r--].src}if(!t)throw new Error("Automatic publicPath is not supported in this browser");t=t.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),a.p=t})(),a.nc=void 0,a(237)})();
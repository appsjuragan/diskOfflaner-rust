(function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))s(r);new MutationObserver(r=>{for(const l of r)if(l.type==="childList")for(const o of l.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&s(o)}).observe(document,{childList:!0,subtree:!0});function n(r){const l={};return r.integrity&&(l.integrity=r.integrity),r.referrerPolicy&&(l.referrerPolicy=r.referrerPolicy),r.crossOrigin==="use-credentials"?l.credentials="include":r.crossOrigin==="anonymous"?l.credentials="omit":l.credentials="same-origin",l}function s(r){if(r.ep)return;r.ep=!0;const l=n(r);fetch(r.href,l)}})();const De=!1,Ie=(e,t)=>e===t,q=Symbol("solid-proxy"),ke=typeof Proxy=="function",Le=Symbol("solid-track"),W={equals:Ie};let $e=Ee;const M=1,X=2,Se={owned:null,cleanups:null,context:null,owner:null};var b=null;let ne=null,je=null,g=null,v=null,C=null,J=0;function G(e,t){const n=g,s=b,r=e.length===0,l=t===void 0?s:t,o=r?Se:{owned:null,cleanups:null,context:l?l.context:null,owner:l},i=r?e:()=>e(()=>P(()=>R(o)));b=o,g=null;try{return V(i,!0)}finally{g=n,b=s}}function F(e,t){t=t?Object.assign({},W,t):W;const n={value:e,observers:null,observerSlots:null,comparator:t.equals||void 0},s=r=>(typeof r=="function"&&(r=r(n.value)),_e(n,r));return[Ae.bind(n),s]}function N(e,t,n){const s=le(e,t,!1,M);U(s)}function Fe(e,t,n){$e=Ve;const s=le(e,t,!1,M);s.user=!0,C?C.push(s):U(s)}function B(e,t,n){n=n?Object.assign({},W,n):W;const s=le(e,t,!0,0);return s.observers=null,s.observerSlots=null,s.comparator=n.equals||void 0,U(s),Ae.bind(s)}function P(e){if(g===null)return e();const t=g;g=null;try{return e()}finally{g=t}}function Be(e){Fe(()=>P(e))}function Re(e){return b===null||(b.cleanups===null?b.cleanups=[e]:b.cleanups.push(e)),e}function Ae(){if(this.sources&&this.state)if(this.state===M)U(this);else{const e=v;v=null,V(()=>Y(this),!1),v=e}if(g){const e=this.observers?this.observers.length:0;g.sources?(g.sources.push(this),g.sourceSlots.push(e)):(g.sources=[this],g.sourceSlots=[e]),this.observers?(this.observers.push(g),this.observerSlots.push(g.sources.length-1)):(this.observers=[g],this.observerSlots=[g.sources.length-1])}return this.value}function _e(e,t,n){let s=e.value;return(!e.comparator||!e.comparator(s,t))&&(e.value=t,e.observers&&e.observers.length&&V(()=>{for(let r=0;r<e.observers.length;r+=1){const l=e.observers[r],o=ne&&ne.running;o&&ne.disposed.has(l),(o?!l.tState:!l.state)&&(l.pure?v.push(l):C.push(l),l.observers&&Ce(l)),o||(l.state=M)}if(v.length>1e6)throw v=[],new Error},!1)),t}function U(e){if(!e.fn)return;R(e);const t=J;ze(e,e.value,t)}function ze(e,t,n){let s;const r=b,l=g;g=b=e;try{s=e.fn(t)}catch(o){return e.pure&&(e.state=M,e.owned&&e.owned.forEach(R),e.owned=null),e.updatedAt=n+1,Ne(o)}finally{g=l,b=r}(!e.updatedAt||e.updatedAt<=n)&&(e.updatedAt!=null&&"observers"in e?_e(e,s):e.value=s,e.updatedAt=n)}function le(e,t,n,s=M,r){const l={fn:e,state:s,updatedAt:null,owned:null,sources:null,sourceSlots:null,cleanups:null,value:t,owner:b,context:b?b.context:null,pure:n};return b===null||b!==Se&&(b.owned?b.owned.push(l):b.owned=[l]),l}function Z(e){if(e.state===0)return;if(e.state===X)return Y(e);if(e.suspense&&P(e.suspense.inFallback))return e.suspense.effects.push(e);const t=[e];for(;(e=e.owner)&&(!e.updatedAt||e.updatedAt<J);)e.state&&t.push(e);for(let n=t.length-1;n>=0;n--)if(e=t[n],e.state===M)U(e);else if(e.state===X){const s=v;v=null,V(()=>Y(e,t[0]),!1),v=s}}function V(e,t){if(v)return e();let n=!1;t||(v=[]),C?n=!0:C=[],J++;try{const s=e();return Ue(n),s}catch(s){n||(C=null),v=null,Ne(s)}}function Ue(e){if(v&&(Ee(v),v=null),e)return;const t=C;C=null,t.length&&V(()=>$e(t),!1)}function Ee(e){for(let t=0;t<e.length;t++)Z(e[t])}function Ve(e){let t,n=0;for(t=0;t<e.length;t++){const s=e[t];s.user?e[n++]=s:Z(s)}for(t=0;t<n;t++)Z(e[t])}function Y(e,t){e.state=0;for(let n=0;n<e.sources.length;n+=1){const s=e.sources[n];if(s.sources){const r=s.state;r===M?s!==t&&(!s.updatedAt||s.updatedAt<J)&&Z(s):r===X&&Y(s,t)}}}function Ce(e){for(let t=0;t<e.observers.length;t+=1){const n=e.observers[t];n.state||(n.state=X,n.pure?v.push(n):C.push(n),n.observers&&Ce(n))}}function R(e){let t;if(e.sources)for(;e.sources.length;){const n=e.sources.pop(),s=e.sourceSlots.pop(),r=n.observers;if(r&&r.length){const l=r.pop(),o=n.observerSlots.pop();s<r.length&&(l.sourceSlots[o]=s,r[s]=l,n.observerSlots[s]=o)}}if(e.tOwned){for(t=e.tOwned.length-1;t>=0;t--)R(e.tOwned[t]);delete e.tOwned}if(e.owned){for(t=e.owned.length-1;t>=0;t--)R(e.owned[t]);e.owned=null}if(e.cleanups){for(t=e.cleanups.length-1;t>=0;t--)e.cleanups[t]();e.cleanups=null}e.state=0}function He(e){return e instanceof Error?e:new Error(typeof e=="string"?e:"Unknown error",{cause:e})}function Ne(e,t=b){throw He(e)}const Ge=Symbol("fallback");function pe(e){for(let t=0;t<e.length;t++)e[t]()}function Ke(e,t,n={}){let s=[],r=[],l=[],o=0,i=t.length>1?[]:null;return Re(()=>pe(l)),()=>{let a=e()||[],f=a.length,u,c;return a[Le],P(()=>{let h,p,w,$,_,S,x,A,E;if(f===0)o!==0&&(pe(l),l=[],s=[],r=[],o=0,i&&(i=[])),n.fallback&&(s=[Ge],r[0]=G(te=>(l[0]=te,n.fallback())),o=1);else if(o===0){for(r=new Array(f),c=0;c<f;c++)s[c]=a[c],r[c]=G(d);o=f}else{for(w=new Array(f),$=new Array(f),i&&(_=new Array(f)),S=0,x=Math.min(o,f);S<x&&s[S]===a[S];S++);for(x=o-1,A=f-1;x>=S&&A>=S&&s[x]===a[A];x--,A--)w[A]=r[x],$[A]=l[x],i&&(_[A]=i[x]);for(h=new Map,p=new Array(A+1),c=A;c>=S;c--)E=a[c],u=h.get(E),p[c]=u===void 0?-1:u,h.set(E,c);for(u=S;u<=x;u++)E=s[u],c=h.get(E),c!==void 0&&c!==-1?(w[c]=r[u],$[c]=l[u],i&&(_[c]=i[u]),c=p[c],h.set(E,c)):l[u]();for(c=S;c<f;c++)c in w?(r[c]=w[c],l[c]=$[c],i&&(i[c]=_[c],i[c](c))):r[c]=G(d);r=r.slice(0,o=f),s=a.slice(0)}return r});function d(h){if(l[c]=h,i){const[p,w]=F(c);return i[c]=w,t(a[c],p)}return t(a[c])}}}function k(e,t){return P(()=>e(t||{}))}function H(){return!0}const re={get(e,t,n){return t===q?n:e.get(t)},has(e,t){return t===q?!0:e.has(t)},set:H,deleteProperty:H,getOwnPropertyDescriptor(e,t){return{configurable:!0,enumerable:!0,get(){return e.get(t)},set:H,deleteProperty:H}},ownKeys(e){return e.keys()}};function se(e){return(e=typeof e=="function"?e():e)?e:{}}function qe(){for(let e=0,t=this.length;e<t;++e){const n=this[e]();if(n!==void 0)return n}}function O(...e){let t=!1;for(let o=0;o<e.length;o++){const i=e[o];t=t||!!i&&q in i,e[o]=typeof i=="function"?(t=!0,B(i)):i}if(ke&&t)return new Proxy({get(o){for(let i=e.length-1;i>=0;i--){const a=se(e[i])[o];if(a!==void 0)return a}},has(o){for(let i=e.length-1;i>=0;i--)if(o in se(e[i]))return!0;return!1},keys(){const o=[];for(let i=0;i<e.length;i++)o.push(...Object.keys(se(e[i])));return[...new Set(o)]}},re);const n={},s=Object.create(null);for(let o=e.length-1;o>=0;o--){const i=e[o];if(!i)continue;const a=Object.getOwnPropertyNames(i);for(let f=a.length-1;f>=0;f--){const u=a[f];if(u==="__proto__"||u==="constructor")continue;const c=Object.getOwnPropertyDescriptor(i,u);if(!s[u])s[u]=c.get?{enumerable:!0,configurable:!0,get:qe.bind(n[u]=[c.get.bind(i)])}:c.value!==void 0?c:void 0;else{const d=n[u];d&&(c.get?d.push(c.get.bind(i)):c.value!==void 0&&d.push(()=>c.value))}}}const r={},l=Object.keys(s);for(let o=l.length-1;o>=0;o--){const i=l[o],a=s[i];a&&a.get?Object.defineProperty(r,i,a):r[i]=a?a.value:void 0}return r}function Pe(e,...t){const n=t.length;if(ke&&q in e){const r=n>1?t.flat():t[0],l=t.map(o=>new Proxy({get(i){return o.includes(i)?e[i]:void 0},has(i){return o.includes(i)&&i in e},keys(){return o.filter(i=>i in e)}},re));return l.push(new Proxy({get(o){return r.includes(o)?void 0:e[o]},has(o){return r.includes(o)?!1:o in e},keys(){return Object.keys(e).filter(o=>!r.includes(o))}},re)),l}const s=[];for(let r=0;r<=n;r++)s[r]={};for(const r of Object.getOwnPropertyNames(e)){let l=n;for(let a=0;a<t.length;a++)if(t[a].includes(r)){l=a;break}const o=Object.getOwnPropertyDescriptor(e,r);!o.get&&!o.set&&o.enumerable&&o.writable&&o.configurable?s[l][r]=o.value:Object.defineProperty(s[l],r,o)}return s}function We(e){const t="fallback"in e&&{fallback:()=>e.fallback};return B(Ke(()=>e.each,e.children,t||void 0))}const Xe=["allowfullscreen","async","alpha","autofocus","autoplay","checked","controls","default","disabled","formnovalidate","hidden","indeterminate","inert","ismap","loop","multiple","muted","nomodule","novalidate","open","playsinline","readonly","required","reversed","seamless","selected","adauctionheaders","browsingtopics","credentialless","defaultchecked","defaultmuted","defaultselected","defer","disablepictureinpicture","disableremoteplayback","preservespitch","shadowrootclonable","shadowrootcustomelementregistry","shadowrootdelegatesfocus","shadowrootserializable","sharedstoragewritable"],Ze=new Set(["className","value","readOnly","noValidate","formNoValidate","isMap","noModule","playsInline","adAuctionHeaders","allowFullscreen","browsingTopics","defaultChecked","defaultMuted","defaultSelected","disablePictureInPicture","disableRemotePlayback","preservesPitch","shadowRootClonable","shadowRootCustomElementRegistry","shadowRootDelegatesFocus","shadowRootSerializable","sharedStorageWritable",...Xe]),Ye=new Set(["innerHTML","textContent","innerText","children"]),Qe=Object.assign(Object.create(null),{className:"class",htmlFor:"for"}),Je=Object.assign(Object.create(null),{class:"className",novalidate:{$:"noValidate",FORM:1},formnovalidate:{$:"formNoValidate",BUTTON:1,INPUT:1},ismap:{$:"isMap",IMG:1},nomodule:{$:"noModule",SCRIPT:1},playsinline:{$:"playsInline",VIDEO:1},readonly:{$:"readOnly",INPUT:1,TEXTAREA:1},adauctionheaders:{$:"adAuctionHeaders",IFRAME:1},allowfullscreen:{$:"allowFullscreen",IFRAME:1},browsingtopics:{$:"browsingTopics",IMG:1},defaultchecked:{$:"defaultChecked",INPUT:1},defaultmuted:{$:"defaultMuted",AUDIO:1,VIDEO:1},defaultselected:{$:"defaultSelected",OPTION:1},disablepictureinpicture:{$:"disablePictureInPicture",VIDEO:1},disableremoteplayback:{$:"disableRemotePlayback",AUDIO:1,VIDEO:1},preservespitch:{$:"preservesPitch",AUDIO:1,VIDEO:1},shadowrootclonable:{$:"shadowRootClonable",TEMPLATE:1},shadowrootdelegatesfocus:{$:"shadowRootDelegatesFocus",TEMPLATE:1},shadowrootserializable:{$:"shadowRootSerializable",TEMPLATE:1},sharedstoragewritable:{$:"sharedStorageWritable",IFRAME:1,IMG:1}});function et(e,t){const n=Je[e];return typeof n=="object"?n[t]?n.$:void 0:n}const tt=new Set(["beforeinput","click","dblclick","contextmenu","focusin","focusout","input","keydown","keyup","mousedown","mousemove","mouseout","mouseover","mouseup","pointerdown","pointermove","pointerout","pointerover","pointerup","touchend","touchmove","touchstart"]),nt=new Set(["altGlyph","altGlyphDef","altGlyphItem","animate","animateColor","animateMotion","animateTransform","circle","clipPath","color-profile","cursor","defs","desc","ellipse","feBlend","feColorMatrix","feComponentTransfer","feComposite","feConvolveMatrix","feDiffuseLighting","feDisplacementMap","feDistantLight","feDropShadow","feFlood","feFuncA","feFuncB","feFuncG","feFuncR","feGaussianBlur","feImage","feMerge","feMergeNode","feMorphology","feOffset","fePointLight","feSpecularLighting","feSpotLight","feTile","feTurbulence","filter","font","font-face","font-face-format","font-face-name","font-face-src","font-face-uri","foreignObject","g","glyph","glyphRef","hkern","image","line","linearGradient","marker","mask","metadata","missing-glyph","mpath","path","pattern","polygon","polyline","radialGradient","rect","set","stop","svg","switch","symbol","text","textPath","tref","tspan","use","view","vkern"]),st={xlink:"http://www.w3.org/1999/xlink",xml:"http://www.w3.org/XML/1998/namespace"},Q=e=>B(()=>e());function it(e,t,n){let s=n.length,r=t.length,l=s,o=0,i=0,a=t[r-1].nextSibling,f=null;for(;o<r||i<l;){if(t[o]===n[i]){o++,i++;continue}for(;t[r-1]===n[l-1];)r--,l--;if(r===o){const u=l<s?i?n[i-1].nextSibling:n[l-i]:a;for(;i<l;)e.insertBefore(n[i++],u)}else if(l===i)for(;o<r;)(!f||!f.has(t[o]))&&t[o].remove(),o++;else if(t[o]===n[l-1]&&n[i]===t[r-1]){const u=t[--r].nextSibling;e.insertBefore(n[i++],t[o++].nextSibling),e.insertBefore(n[--l],u),t[r]=n[l]}else{if(!f){f=new Map;let c=i;for(;c<l;)f.set(n[c],c++)}const u=f.get(t[o]);if(u!=null)if(i<u&&u<l){let c=o,d=1,h;for(;++c<r&&c<l&&!((h=f.get(t[c]))==null||h!==u+d);)d++;if(d>u-i){const p=t[o];for(;i<u;)e.insertBefore(n[i++],p)}else e.replaceChild(n[i++],t[o++])}else o++;else t[o++].remove()}}}const be="_$DX_DELEGATE";function rt(e,t,n,s={}){let r;return G(l=>{r=l,t===document?e():m(t,e(),t.firstChild?null:void 0,n)},s.owner),()=>{r(),t.textContent=""}}function D(e,t,n,s){let r;const l=()=>{const i=document.createElement("template");return i.innerHTML=e,i.content.firstChild},o=()=>(r||(r=l())).cloneNode(!0);return o.cloneNode=o,o}function ee(e,t=window.document){const n=t[be]||(t[be]=new Set);for(let s=0,r=e.length;s<r;s++){const l=e[s];n.has(l)||(n.add(l),t.addEventListener(l,gt))}}function T(e,t,n){n==null?e.removeAttribute(t):e.setAttribute(t,n)}function ot(e,t,n,s){s==null?e.removeAttributeNS(t,n):e.setAttributeNS(t,n,s)}function lt(e,t,n){n?e.setAttribute(t,""):e.removeAttribute(t)}function ae(e,t){t==null?e.removeAttribute("class"):e.className=t}function at(e,t,n,s){if(s)Array.isArray(n)?(e[`$$${t}`]=n[0],e[`$$${t}Data`]=n[1]):e[`$$${t}`]=n;else if(Array.isArray(n)){const r=n[0];e.addEventListener(t,n[0]=l=>r.call(e,n[1],l))}else e.addEventListener(t,n,typeof n!="function"&&n)}function ct(e,t,n={}){const s=Object.keys(t||{}),r=Object.keys(n);let l,o;for(l=0,o=r.length;l<o;l++){const i=r[l];!i||i==="undefined"||t[i]||(ye(e,i,!1),delete n[i])}for(l=0,o=s.length;l<o;l++){const i=s[l],a=!!t[i];!i||i==="undefined"||n[i]===a||!a||(ye(e,i,!0),n[i]=a)}return n}function ft(e,t,n){if(!t)return n?T(e,"style"):t;const s=e.style;if(typeof t=="string")return s.cssText=t;typeof n=="string"&&(s.cssText=n=void 0),n||(n={}),t||(t={});let r,l;for(l in n)t[l]==null&&s.removeProperty(l),delete n[l];for(l in t)r=t[l],r!==n[l]&&(s.setProperty(l,r),n[l]=r);return n}function me(e,t,n){n!=null?e.style.setProperty(t,n):e.style.removeProperty(t)}function Oe(e,t={},n,s){const r={};return s||N(()=>r.children=z(e,t.children,r.children)),N(()=>typeof t.ref=="function"&&ut(t.ref,e)),N(()=>dt(e,t,n,!0,r,!0)),r}function ut(e,t,n){return P(()=>e(t,n))}function m(e,t,n,s){if(n!==void 0&&!s&&(s=[]),typeof t!="function")return z(e,t,s,n);N(r=>z(e,t(),r,n),s)}function dt(e,t,n,s,r={},l=!1){t||(t={});for(const o in r)if(!(o in t)){if(o==="children")continue;r[o]=ve(e,o,null,r[o],n,l,t)}for(const o in t){if(o==="children")continue;const i=t[o];r[o]=ve(e,o,i,r[o],n,l,t)}}function ht(e){return e.toLowerCase().replace(/-([a-z])/g,(t,n)=>n.toUpperCase())}function ye(e,t,n){const s=t.trim().split(/\s+/);for(let r=0,l=s.length;r<l;r++)e.classList.toggle(s[r],n)}function ve(e,t,n,s,r,l,o){let i,a,f,u,c;if(t==="style")return ft(e,n,s);if(t==="classList")return ct(e,n,s);if(n===s)return s;if(t==="ref")l||n(e);else if(t.slice(0,3)==="on:"){const d=t.slice(3);s&&e.removeEventListener(d,s,typeof s!="function"&&s),n&&e.addEventListener(d,n,typeof n!="function"&&n)}else if(t.slice(0,10)==="oncapture:"){const d=t.slice(10);s&&e.removeEventListener(d,s,!0),n&&e.addEventListener(d,n,!0)}else if(t.slice(0,2)==="on"){const d=t.slice(2).toLowerCase(),h=tt.has(d);if(!h&&s){const p=Array.isArray(s)?s[0]:s;e.removeEventListener(d,p)}(h||n)&&(at(e,d,n,h),h&&ee([d]))}else if(t.slice(0,5)==="attr:")T(e,t.slice(5),n);else if(t.slice(0,5)==="bool:")lt(e,t.slice(5),n);else if((c=t.slice(0,5)==="prop:")||(f=Ye.has(t))||!r&&((u=et(t,e.tagName))||(a=Ze.has(t)))||(i=e.nodeName.includes("-")||"is"in o))c&&(t=t.slice(5),a=!0),t==="class"||t==="className"?ae(e,n):i&&!a&&!f?e[ht(t)]=n:e[u||t]=n;else{const d=r&&t.indexOf(":")>-1&&st[t.split(":")[0]];d?ot(e,d,t,n):T(e,Qe[t]||t,n)}return n}function gt(e){let t=e.target;const n=`$$${e.type}`,s=e.target,r=e.currentTarget,l=a=>Object.defineProperty(e,"target",{configurable:!0,value:a}),o=()=>{const a=t[n];if(a&&!t.disabled){const f=t[`${n}Data`];if(f!==void 0?a.call(t,f,e):a.call(t,e),e.cancelBubble)return}return t.host&&typeof t.host!="string"&&!t.host._$host&&t.contains(e.target)&&l(t.host),!0},i=()=>{for(;o()&&(t=t._$host||t.parentNode||t.host););};if(Object.defineProperty(e,"currentTarget",{configurable:!0,get(){return t||document}}),e.composedPath){const a=e.composedPath();l(a[0]);for(let f=0;f<a.length-2&&(t=a[f],!!o());f++){if(t._$host){t=t._$host,i();break}if(t.parentNode===r)break}}else i();l(s)}function z(e,t,n,s,r){for(;typeof n=="function";)n=n();if(t===n)return n;const l=typeof t,o=s!==void 0;if(e=o&&n[0]&&n[0].parentNode||e,l==="string"||l==="number"){if(l==="number"&&(t=t.toString(),t===n))return n;if(o){let i=n[0];i&&i.nodeType===3?i.data!==t&&(i.data=t):i=document.createTextNode(t),n=I(e,n,s,i)}else n!==""&&typeof n=="string"?n=e.firstChild.data=t:n=e.textContent=t}else if(t==null||l==="boolean")n=I(e,n,s);else{if(l==="function")return N(()=>{let i=t();for(;typeof i=="function";)i=i();n=z(e,i,n,s)}),()=>n;if(Array.isArray(t)){const i=[],a=n&&Array.isArray(n);if(oe(i,t,n,r))return N(()=>n=z(e,i,n,s,!0)),()=>n;if(i.length===0){if(n=I(e,n,s),o)return n}else a?n.length===0?we(e,i,s):it(e,n,i):(n&&I(e),we(e,i));n=i}else if(t.nodeType){if(Array.isArray(n)){if(o)return n=I(e,n,s,t);I(e,n,null,t)}else n==null||n===""||!e.firstChild?e.appendChild(t):e.replaceChild(t,e.firstChild);n=t}}return n}function oe(e,t,n,s){let r=!1;for(let l=0,o=t.length;l<o;l++){let i=t[l],a=n&&n[e.length],f;if(!(i==null||i===!0||i===!1))if((f=typeof i)=="object"&&i.nodeType)e.push(i);else if(Array.isArray(i))r=oe(e,i,a)||r;else if(f==="function")if(s){for(;typeof i=="function";)i=i();r=oe(e,Array.isArray(i)?i:[i],Array.isArray(a)?a:[a])||r}else e.push(i),r=!0;else{const u=String(i);a&&a.nodeType===3&&a.data===u?e.push(a):e.push(document.createTextNode(u))}}return r}function we(e,t,n=null){for(let s=0,r=t.length;s<r;s++)e.insertBefore(t[s],n)}function I(e,t,n,s){if(n===void 0)return e.textContent="";const r=s||document.createTextNode("");if(t.length){let l=!1;for(let o=t.length-1;o>=0;o--){const i=t[o];if(r!==i){const a=i.parentNode===e;!l&&!o?a?e.replaceChild(r,i):e.insertBefore(r,n):a&&i.remove()}else l=!0}}else e.insertBefore(r,n);return[r]}const pt="http://www.w3.org/2000/svg";function bt(e,t=!1,n=void 0){return t?document.createElementNS(pt,e):document.createElement(e,{is:n})}function mt(e,t){const n=B(e);return B(()=>{const s=n();switch(typeof s){case"function":return P(()=>s(t));case"string":const r=nt.has(s),l=bt(s,r,P(()=>t.is));return Oe(l,t,r),l}})}function yt(e){const[,t]=Pe(e,["component"]);return mt(()=>e.component,t)}async function ie(e,t={},n){return window.__TAURI_INTERNALS__.invoke(e,t,n)}/**
* @license lucide-solid v0.475.0 - ISC
*
* This source code is licensed under the ISC license.
* See the LICENSE file in the root directory of this source tree.
*/var vt={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"},L=vt,wt=D("<svg>"),xt=e=>e.replace(/([a-z0-9])([A-Z])/g,"$1-$2").toLowerCase(),kt=(...e)=>e.filter((t,n,s)=>!!t&&t.trim()!==""&&s.indexOf(t)===n).join(" ").trim(),$t=e=>{const[t,n]=Pe(e,["color","size","strokeWidth","children","class","name","iconNode","absoluteStrokeWidth"]);return(()=>{var s=wt();return Oe(s,O(L,{get width(){return t.size??L.width},get height(){return t.size??L.height},get stroke(){return t.color??L.stroke},get"stroke-width"(){return Q(()=>!!t.absoluteStrokeWidth)()?Number(t.strokeWidth??L["stroke-width"])*24/Number(t.size):Number(t.strokeWidth??L["stroke-width"])},get class(){return kt("lucide","lucide-icon",t.name!=null?`lucide-${xt(t==null?void 0:t.name)}`:void 0,t.class!=null?t.class:"")}},n),!0,!0),m(s,k(We,{get each(){return t.iconNode},children:([r,l])=>k(yt,O({component:r},l))})),s})()},j=$t,St=[["ellipse",{cx:"12",cy:"5",rx:"9",ry:"3",key:"msslwz"}],["path",{d:"M3 5V19A9 3 0 0 0 21 19V5",key:"1wlel7"}],["path",{d:"M3 12A9 3 0 0 0 21 12",key:"mv7ke4"}]],At=e=>k(j,O(e,{name:"Database",iconNode:St})),_t=At,Et=[["path",{d:"M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z",key:"1rqfz7"}],["path",{d:"M14 2v4a2 2 0 0 0 2 2h4",key:"tnqrlb"}],["path",{d:"M10 9H8",key:"b1mrlr"}],["path",{d:"M16 13H8",key:"t4e002"}],["path",{d:"M16 17H8",key:"z1uh3a"}]],Ct=e=>k(j,O(e,{name:"FileText",iconNode:Et})),Nt=Ct,Pt=[["line",{x1:"22",x2:"2",y1:"12",y2:"12",key:"1y58io"}],["path",{d:"M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z",key:"oot6mr"}],["line",{x1:"6",x2:"6.01",y1:"16",y2:"16",key:"sgf278"}],["line",{x1:"10",x2:"10.01",y1:"16",y2:"16",key:"1l4acy"}]],Ot=e=>k(j,O(e,{name:"HardDrive",iconNode:Pt})),K=Ot,Mt=[["circle",{cx:"12",cy:"12",r:"10",key:"1mglay"}],["path",{d:"M12 16v-4",key:"1dtifu"}],["path",{d:"M12 8h.01",key:"e9boi3"}]],Tt=e=>k(j,O(e,{name:"Info",iconNode:Mt})),Dt=Tt,It=[["path",{d:"M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z",key:"1qme2f"}],["circle",{cx:"12",cy:"12",r:"3",key:"1v7zrd"}]],Lt=e=>k(j,O(e,{name:"Settings",iconNode:It})),jt=Lt,Ft=[["circle",{cx:"10",cy:"7",r:"1",key:"dypaad"}],["circle",{cx:"4",cy:"20",r:"1",key:"22iqad"}],["path",{d:"M4.7 19.3 19 5",key:"1enqfc"}],["path",{d:"m21 3-3 1 2 2Z",key:"d3ov82"}],["path",{d:"M9.26 7.68 5 12l2 5",key:"1esawj"}],["path",{d:"m10 14 5 2 3.5-3.5",key:"v8oal5"}],["path",{d:"m18 12 1-1 1 1-1 1Z",key:"1bh22v"}]],Bt=e=>k(j,O(e,{name:"Usb",iconNode:Ft})),xe=Bt,Rt=D(`<aside class=sidebar><div class=sidebar-logo><div class=logo-icon></div><span class=logo-text>DiskOfflaner</span></div><nav class=sidebar-nav></nav><style jsx>
        .sidebar {
          width: 220px;
          background-color: var(--bg-sidebar);
          padding: 24px 16px;
          border-right: 1px solid var(--border-sidebar);
          display: flex;
          flex-direction: column;
        }

        .sidebar-logo {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 12px;
          margin-bottom: 32px;
        }

        .logo-icon {
          width: 32px;
          height: 32px;
          background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
          border-radius: 8px;
        }

        .logo-text {
          font-size: 18px;
          font-weight: 600;
        }

        .sidebar-nav {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }

        .nav-item {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 12px 16px;
          border-radius: 8px;
          border: none;
          background: transparent;
          color: var(--text-secondary);
          transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
          text-align: left;
          width: 100%;
        }

        .nav-item:hover {
          background-color: var(--bg-sidebar-item-hover);
          color: var(--text-primary);
        }

        .nav-item:hover .nav-icon {
          transform: translateX(2px);
          opacity: 1;
        }

        .nav-item.active {
          background-color: var(--bg-sidebar-item);
          color: var(--text-primary);
          font-weight: 600;
        }

        .nav-icon {
          stroke-width: 2px;
          opacity: 0.7;
          transition: transform 0.2s, opacity 0.2s;
        }
      `),zt=D("<button><span class=nav-label>");function Ut(e){const t=[{id:"drives",label:"Drives",icon:K},{id:"settings",label:"Settings",icon:jt},{id:"logs",label:"Logs",icon:Nt},{id:"info",label:"System Info",icon:Dt}];return(()=>{var n=Rt(),s=n.firstChild,r=s.nextSibling;return m(r,()=>t.map(l=>(()=>{var o=zt(),i=o.firstChild;return o.$$click=()=>e.setActivePage(l.id),m(o,k(l.icon,{class:"nav-icon",size:20}),i),m(i,()=>l.label),N(()=>ae(o,`nav-item ${e.activePage===l.id?"active":""}`)),o})())),n})()}ee(["click"]);var Vt=D(`<div class=disk-card><header class=card-header><div class=card-title><div class=icon-wrapper></div><span class=disk-name></span></div><button></button></header><div class=card-body><div class=info-row><span class=label>Model</span><span class=value></span></div><div class=info-row><span class=label>Capacity</span><span class=value></span></div><div class=info-row><span class=label>Health</span><span class="value health"></span></div><div class=info-row><span class=label>Serial</span><span class="value serial"></span></div></div><style jsx>
        .disk-card {
          background: var(--bg-card);
          border: 1px solid var(--border-card);
          border-radius: 16px;
          padding: 20px;
          backdrop-filter: blur(10px);
          -webkit-backdrop-filter: blur(10px);
          transition: transform 0.25s var(--ease-smooth), box-shadow 0.25s var(--ease-smooth), background 0.25s;
          position: relative;
          overflow: hidden;
        }

        .disk-card:hover {
          background: var(--bg-card-hover);
          transform: translateY(-2px);
          box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
        }

        .disk-card[data-status="offline"] {
          border-color: rgba(239, 68, 68, 0.3);
          background: rgba(60, 40, 45, 0.6);
        }

        .disk-card[data-status="offline"]:hover {
          background: rgba(70, 45, 50, 0.7);
        }

        /* Header */
        .card-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 20px;
          padding-bottom: 12px;
          border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }

        .card-title {
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .disk-name {
          font-size: 16px;
          font-weight: 600;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          max-width: 180px;
        }

        .icon-wrapper {
          display: flex;
          align-items: center;
          justify-content: center;
        }

        /* Status Badge */
        .status-badge {
          padding: 4px 12px;
          border-radius: 12px;
          font-size: 11px;
          font-weight: 700;
          letter-spacing: 0.05em;
          text-transform: uppercase;
        }

        .status-badge.online {
          background: rgba(74, 222, 128, 0.15);
          color: var(--status-online);
          border: 1px solid rgba(74, 222, 128, 0.3);
          box-shadow: 0 0 0 0 rgba(74, 222, 128, 0.4);
          animation: pulse-glow 3s infinite;
        }

        .status-badge.offline {
          background: rgba(239, 68, 68, 0.15);
          color: var(--status-offline);
          border: 1px solid rgba(239, 68, 68, 0.3);
        }

        @keyframes pulse-glow {
          0%, 100% { box-shadow: 0 0 0 0 rgba(74, 222, 128, 0); }
          50% { box-shadow: 0 0 8px 0 rgba(74, 222, 128, 0.2); }
        }

        /* Body */
        .card-body {
          display: flex;
          flex-direction: column;
          gap: 10px;
        }

        .info-row {
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .label {
          font-size: 13px;
          color: var(--text-secondary);
          font-weight: 500;
        }

        .value {
          font-size: 14px;
          color: var(--text-value);
          font-weight: 600;
        }

        .value.serial {
          font-family: var(--font-mono);
          letter-spacing: 0.5px;
          font-size: 13px;
        }
      `);function Ht(e){const{disk:t}=e,n=()=>{switch(t.disk_type){case"NVMe":return K;case"SSD":return K;case"USBFlash":return xe;case"HDD":return _t;case"ExtHDD":return xe;default:return K}},s=o=>{if(!o)return"0 B";const i=1024,a=["B","KB","MB","GB","TB"],f=Math.floor(Math.log(o)/Math.log(i));return parseFloat((o/Math.pow(i,f)).toFixed(2))+" "+a[f]},r=()=>{switch(t.disk_type){case"NVMe":return"var(--icon-nvme)";case"SSD":return"var(--icon-ssd)";case"USBFlash":return"var(--icon-usb)";case"HDD":return"var(--icon-hdd)";default:return"var(--text-secondary)"}},l=n();return(()=>{var o=Vt(),i=o.firstChild,a=i.firstChild,f=a.firstChild,u=f.nextSibling,c=a.nextSibling,d=i.nextSibling,h=d.firstChild,p=h.firstChild,w=p.nextSibling,$=h.nextSibling,_=$.firstChild,S=_.nextSibling,x=$.nextSibling,A=x.firstChild,E=A.nextSibling,te=x.nextSibling,Me=te.firstChild,Te=Me.nextSibling;return m(f,k(l,{size:24})),m(u,()=>t.model),c.$$click=y=>{y.stopPropagation(),e.onToggle&&e.onToggle()},m(c,()=>t.is_online?"ONLINE":"OFFLINE"),m(w,()=>t.model),m(S,()=>s(t.size_bytes)),m(E,(()=>{var y=Q(()=>t.health_percentage!==null);return()=>y()?`${t.health_percentage}%`:"N/A"})()),m(Te,()=>t.serial_number||"N/A"),N(y=>{var ce=t.is_online?"online":"offline",fe=r(),ue=t.model,de=`status-badge ${t.is_online?"online":"offline"}`,he=t.is_online?"Click to set Offline":"Click to set Online",ge=(t.health_percentage||100)>=70?"var(--health-good)":"var(--health-critical)";return ce!==y.e&&T(o,"data-status",y.e=ce),fe!==y.t&&me(f,"color",y.t=fe),ue!==y.a&&T(u,"title",y.a=ue),de!==y.o&&ae(c,y.o=de),he!==y.i&&T(c,"title",y.i=he),ge!==y.n&&me(E,"color",y.n=ge),y},{e:void 0,t:void 0,a:void 0,o:void 0,i:void 0,n:void 0}),o})()}ee(["click"]);var Gt=D(`<div class=app-container><main class=main-content><header class=main-header><h1 class=main-title></h1><button class=refresh-btn>Refresh</button></header></main><style jsx>
        .app-container {
          display: flex;
          height: 100vh;
          background-color: var(--bg-app);
          color: var(--text-primary);
        }

        .main-content {
          flex: 1;
          padding: 32px 40px;
          overflow-y: auto;
          background-color: var(--bg-main);
        }

        .main-header {
          margin-bottom: 32px;
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .main-title {
          font-size: 32px;
          font-weight: 700;
          letter-spacing: -0.02em;
        }

        .refresh-btn {
          padding: 8px 16px;
          background: var(--bg-card);
          border: 1px solid var(--border-card);
          color: var(--text-primary);
          border-radius: 8px;
          transition: all 0.2s;
        }

        .refresh-btn:hover:not(:disabled) {
          background: var(--bg-card-hover);
        }

        .disk-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
          gap: 24px;
        }

        @media (max-width: 879px) {
          .disk-grid {
            grid-template-columns: 1fr;
          }
        }
      `),Kt=D("<div class=disk-grid>"),qt=D("<div class=loading>Loading disks...");function Wt(){const[e,t]=F("drives"),[n]=F("dark"),[s,r]=F([]),[l,o]=F(!0),i=async()=>{try{const f=await ie("enumerate_disks_command");console.log("Disks fetched:",f),r(f)}catch(f){console.error("Failed to fetch disks:",f)}finally{o(!1)}},a=async f=>{try{o(!0),f.is_online?await ie("set_disk_offline_command",{diskId:f.id}):await ie("set_disk_online_command",{diskId:f.id}),await i()}catch(u){console.error("Failed to toggle disk:",u),o(!1)}};return Be(()=>{i()}),(()=>{var f=Gt(),u=f.firstChild,c=u.firstChild,d=c.firstChild,h=d.nextSibling;return m(f,k(Ut,{get activePage(){return e()},setActivePage:t}),u),m(d,()=>e()==="drives"&&"Dashboard",null),m(d,()=>e()==="settings"&&"Settings",null),m(d,()=>e()==="logs"&&"Logs",null),m(d,()=>e()==="info"&&"System Info",null),h.$$click=i,m(u,(()=>{var p=Q(()=>e()==="drives");return()=>p()&&(()=>{var w=Kt();return m(w,(()=>{var $=Q(()=>!!(l()&&s().length===0));return()=>$()?qt():s().map(_=>k(Ht,{disk:_,onToggle:()=>a(_)}))})()),w})()})(),null),N(p=>{var w=n(),$=l();return w!==p.e&&T(f,"data-theme",p.e=w),$!==p.t&&(h.disabled=p.t=$),p},{e:void 0,t:void 0}),f})()}ee(["click"]);const Xt=document.getElementById("root");rt(()=>k(Wt,{}),Xt);

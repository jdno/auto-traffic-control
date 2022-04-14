"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[518],{3905:function(e,t,n){n.d(t,{Zo:function(){return s},kt:function(){return d}});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},a=Object.keys(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(r=0;r<a.length;r++)n=a[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var u=r.createContext({}),c=function(e){var t=r.useContext(u),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},s=function(e){var t=c(e.components);return r.createElement(u.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},f=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,a=e.originalType,u=e.parentName,s=l(e,["components","mdxType","originalType","parentName"]),f=c(n),d=o,m=f["".concat(u,".").concat(d)]||f[d]||p[d]||a;return n?r.createElement(m,i(i({ref:t},s),{},{components:n})):r.createElement(m,i({ref:t},s))}));function d(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=n.length,i=new Array(a);i[0]=f;var l={};for(var u in t)hasOwnProperty.call(t,u)&&(l[u]=t[u]);l.originalType=e,l.mdxType="string"==typeof e?e:o,i[1]=l;for(var c=2;c<a;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}f.displayName="MDXCreateElement"},6421:function(e,t,n){n.r(t),n.d(t,{assets:function(){return s},contentTitle:function(){return u},default:function(){return d},frontMatter:function(){return l},metadata:function(){return c},toc:function(){return p}});var r=n(7462),o=n(3366),a=(n(7294),n(3905)),i=["components"],l={},u="Rules",c={unversionedId:"rules",id:"rules",title:"Rules",description:"Auto Traffic Control has a few simple rules.",source:"@site/docs/rules.md",sourceDirName:".",slug:"/rules",permalink:"/docs/rules",editUrl:"https://github.com/jdno/auto-traffic-control/tree/main/docs/docs/rules.md",tags:[],version:"current",frontMatter:{},sidebar:"docs",previous:{title:"Quickstart",permalink:"/docs/"},next:{title:"Coordinates",permalink:"/docs/coordinates"}},s={},p=[{value:"Failure",id:"failure",level:2},{value:"Scoring",id:"scoring",level:2},{value:"Flight Plans",id:"flight-plans",level:2}],f={toc:p};function d(e){var t=e.components,n=(0,o.Z)(e,i);return(0,a.kt)("wrapper",(0,r.Z)({},f,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"rules"},"Rules"),(0,a.kt)("p",null,(0,a.kt)("strong",{parentName:"p"},"Auto Traffic Control")," has a few simple rules."),(0,a.kt)("h2",{id:"failure"},"Failure"),(0,a.kt)("p",null,"The game ends when two airplanes get too close to each other."),(0,a.kt)("p",null,"A reasonable approximation of ",(0,a.kt)("em",{parentName:"p"},"too close")," is less than 32 pixels of distance\nbetween them."),(0,a.kt)("h2",{id:"scoring"},"Scoring"),(0,a.kt)("p",null,"You score a point for each airplane that lands on a matching airport."),(0,a.kt)("p",null,(0,a.kt)("em",{parentName:"p"},"Matching")," means that the airplane and the airport have the same\n",(0,a.kt)("a",{parentName:"p",href:"/docs/api/types#tag"},(0,a.kt)("inlineCode",{parentName:"a"},"tag")),"."),(0,a.kt)("h2",{id:"flight-plans"},"Flight Plans"),(0,a.kt)("p",null,"Flight plans are only valid if they match the following constraints. If a flight\nplan violates a constraint, the corresponding validation error (in parentheses)\nis returned by the API."),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Every node must be within the bounds of the map (",(0,a.kt)("inlineCode",{parentName:"li"},"NODE_OUTSIDE_MAP"),")"),(0,a.kt)("li",{parentName:"ul"},"The next node must be a neighbor of the current node (",(0,a.kt)("inlineCode",{parentName:"li"},"INVALID_STEP"),")"),(0,a.kt)("li",{parentName:"ul"},"The next node must not be the previous node, i.e. airplanes cannot fly back\nand forth between two nodes (",(0,a.kt)("inlineCode",{parentName:"li"},"SHARP_TURN"),")"),(0,a.kt)("li",{parentName:"ul"},"The flight plan must continue with the first node of the previous plan (",(0,a.kt)("inlineCode",{parentName:"li"},"INVALID_START"),")"),(0,a.kt)("li",{parentName:"ul"},"The flight plan must not contain ",(0,a.kt)("inlineCode",{parentName:"li"},"restricted")," nodes (",(0,a.kt)("inlineCode",{parentName:"li"},"RESTRICTED_NODE"),")")))}d.isMDXComponent=!0}}]);
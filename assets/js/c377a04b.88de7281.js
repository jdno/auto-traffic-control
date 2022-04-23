"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[971],{3905:function(t,e,n){n.d(e,{Zo:function(){return p},kt:function(){return h}});var r=n(7294);function a(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function o(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);e&&(r=r.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,r)}return n}function i(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?o(Object(n),!0).forEach((function(e){a(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function l(t,e){if(null==t)return{};var n,r,a=function(t,e){if(null==t)return{};var n,r,a={},o=Object.keys(t);for(r=0;r<o.length;r++)n=o[r],e.indexOf(n)>=0||(a[n]=t[n]);return a}(t,e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(t);for(r=0;r<o.length;r++)n=o[r],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(a[n]=t[n])}return a}var u=r.createContext({}),c=function(t){var e=r.useContext(u),n=e;return t&&(n="function"==typeof t?t(e):i(i({},e),t)),n},p=function(t){var e=c(t.components);return r.createElement(u.Provider,{value:e},t.children)},s={inlineCode:"code",wrapper:function(t){var e=t.children;return r.createElement(r.Fragment,{},e)}},m=r.forwardRef((function(t,e){var n=t.components,a=t.mdxType,o=t.originalType,u=t.parentName,p=l(t,["components","mdxType","originalType","parentName"]),m=c(n),h=a,d=m["".concat(u,".").concat(h)]||m[h]||s[h]||o;return n?r.createElement(d,i(i({ref:e},p),{},{components:n})):r.createElement(d,i({ref:e},p))}));function h(t,e){var n=arguments,a=e&&e.mdxType;if("string"==typeof t||a){var o=n.length,i=new Array(o);i[0]=m;var l={};for(var u in e)hasOwnProperty.call(e,u)&&(l[u]=e[u]);l.originalType=t,l.mdxType="string"==typeof t?t:a,i[1]=l;for(var c=2;c<o;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},1269:function(t,e,n){n.r(e),n.d(e,{assets:function(){return p},contentTitle:function(){return u},default:function(){return h},frontMatter:function(){return l},metadata:function(){return c},toc:function(){return s}});var r=n(7462),a=n(3366),o=(n(7294),n(3905)),i=["components"],l={title:"Quickstart"},u="Quickstart",c={unversionedId:"index",id:"index",title:"Quickstart",description:"\ud83d\udc4b Welcome",source:"@site/docs/index.md",sourceDirName:".",slug:"/",permalink:"/docs/",editUrl:"https://github.com/jdno/auto-traffic-control/tree/main/docs/docs/index.md",tags:[],version:"current",frontMatter:{title:"Quickstart"},sidebar:"docs",next:{title:"Introduction",permalink:"/docs/tutorial/"}},p={},s=[{value:"\ud83d\udc4b Welcome",id:"-welcome",level:2},{value:"Install the Game",id:"install-the-game",level:2},{value:"Set Up Your Program",id:"set-up-your-program",level:2},{value:"Rust",id:"rust",level:3},{value:"Download the Client Library",id:"download-the-client-library",level:2},{value:"Connect to the Server",id:"connect-to-the-server",level:2},{value:"Start Playing",id:"start-playing",level:2}],m={toc:s};function h(t){var e=t.components,n=(0,a.Z)(t,i);return(0,o.kt)("wrapper",(0,r.Z)({},m,n,{components:e,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"quickstart"},"Quickstart"),(0,o.kt)("h2",{id:"-welcome"},"\ud83d\udc4b Welcome"),(0,o.kt)("p",null,"We're super happy that you're interested in our game."),(0,o.kt)("p",null,"The ",(0,o.kt)("a",{parentName:"p",href:"/docs"},"Getting Started")," guide shows you how to install the game, set up a client\nlibrary, and start playing."),(0,o.kt)("p",null,"If you are missing anything in the guide, let us know by opening an issue on\n",(0,o.kt)("a",{parentName:"p",href:"https://github.com/jdno/auto-traffic-control"},"GitHub"),"."),(0,o.kt)("h2",{id:"install-the-game"},"Install the Game"),(0,o.kt)("p",null,"The recommended way to install the game is through the ",(0,o.kt)("a",{parentName:"p",href:"https://itch.io"},"itch.io")," app, which runs\non Linux, macOS, and Windows. You will get updates for the game through the app\nas well, making it super easy to play the best\u2122 version of the game."),(0,o.kt)("iframe",{src:"https://itch.io/embed/1463989?link_color=56a9de",width:"552",height:"167",frameborder:"0"},(0,o.kt)("a",{href:"https://jdno.itch.io/auto-traffic-control"},"Auto Traffic Control by jdno")),(0,o.kt)("h2",{id:"set-up-your-program"},"Set Up Your Program"),(0,o.kt)("p",null,"When designing ",(0,o.kt)("strong",{parentName:"p"},"Auto Traffic Control"),", we wanted to give you maximum freedom\nover your own code. Which means this getting started guide can only show you an\nexample that you can copy, tweak, or ignore. Whatever you choose to do, the goal\nof this step is to set up a runnable program."),(0,o.kt)("p",null,"Below is an example for getting started with Rust."),(0,o.kt)("h3",{id:"rust"},"Rust"),(0,o.kt)("p",null,"Creating an executable program in Rust is as simple as running ",(0,o.kt)("inlineCode",{parentName:"p"},"cargo"),":"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-shell"},"cargo new my-atc-program\n")),(0,o.kt)("p",null,"You can then ",(0,o.kt)("inlineCode",{parentName:"p"},"cd")," into the newly created directory and run the program with:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-shell"},"cargo run\n")),(0,o.kt)("h2",{id:"download-the-client-library"},"Download the Client Library"),(0,o.kt)("p",null,"The game provides an API that you can use to query the state of the game and\nissue commands. The API is built with ",(0,o.kt)("a",{parentName:"p",href:"https://grpc.io/"},"gRPC"),", and any programming language that\nsupports gRPC can be used to play the game."),(0,o.kt)("p",null,"We publish official client libraries for the following languages. Simply add the\npackage as a dependency to your project."),(0,o.kt)("table",null,(0,o.kt)("thead",{parentName:"table"},(0,o.kt)("tr",{parentName:"thead"},(0,o.kt)("th",{parentName:"tr",align:null},"Language"),(0,o.kt)("th",{parentName:"tr",align:null},"Package"))),(0,o.kt)("tbody",{parentName:"table"},(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},"Node"),(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("a",{parentName:"td",href:"https://www.npmjs.com/package/auto-traffic-control"},(0,o.kt)("img",{parentName:"a",src:"https://img.shields.io/npm/v/auto-traffic-control",alt:"npm"})))),(0,o.kt)("tr",{parentName:"tbody"},(0,o.kt)("td",{parentName:"tr",align:null},"Rust"),(0,o.kt)("td",{parentName:"tr",align:null},(0,o.kt)("a",{parentName:"td",href:"https://crates.io/crates/auto-traffic-control"},(0,o.kt)("img",{parentName:"a",src:"https://img.shields.io/crates/v/auto-traffic-control",alt:"Crates.io"})))))),(0,o.kt)("p",null,"Missing your preferred programming language? ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/jdno/auto-traffic-control"},"Open an issue on GitHub"),"\nto let us know. Or use the ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/jdno/auto-traffic-control/tree/main/api"},"Protocol Buffers")," to generate your own\nbindings."),(0,o.kt)("h2",{id:"connect-to-the-server"},"Connect to the Server"),(0,o.kt)("p",null,"When you start the game, it runs a ",(0,o.kt)("a",{parentName:"p",href:"https://grpc.io/"},"gRPC")," server in the background. You can\nconnect to the server at the following address:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-text"},"http://localhost:4747\n")),(0,o.kt)("p",null,"How you create a gRPC client depends on the programming language and library\nthat you are using. Check out their documentation for instructions. In ",(0,o.kt)("inlineCode",{parentName:"p"},"Rust"),",\nit might look like this:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-rust"},'let mut game_service = GameServiceClient::connect("http://localhost:4747").await?;\n')),(0,o.kt)("h2",{id:"start-playing"},"Start Playing"),(0,o.kt)("p",null,"Now that you have an executable program, a client library for th ",(0,o.kt)("a",{parentName:"p",href:"https://grpc.io/"},"gRPC")," API, and\na connection to the server, you can start playing the game by sending the\n",(0,o.kt)("inlineCode",{parentName:"p"},"StartGame")," request."),(0,o.kt)("p",null,"Have fun exploring the game!"))}h.isMDXComponent=!0}}]);
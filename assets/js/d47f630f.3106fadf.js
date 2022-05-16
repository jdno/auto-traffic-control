"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[260],{3905:function(e,n,t){t.d(n,{Zo:function(){return s},kt:function(){return m}});var a=t(7294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function i(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function o(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?i(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):i(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function l(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)t=i[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var p=a.createContext({}),d=function(e){var n=a.useContext(p),t=n;return e&&(t="function"==typeof e?e(n):o(o({},n),e)),t},s=function(e){var n=d(e.components);return a.createElement(p.Provider,{value:n},e.children)},c={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},u=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,s=l(e,["components","mdxType","originalType","parentName"]),u=d(t),m=r,g=u["".concat(p,".").concat(m)]||u[m]||c[m]||i;return t?a.createElement(g,o(o({ref:n},s),{},{components:t})):a.createElement(g,o({ref:n},s))}));function m(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var i=t.length,o=new Array(i);o[0]=u;var l={};for(var p in n)hasOwnProperty.call(n,p)&&(l[p]=n[p]);l.originalType=e,l.mdxType="string"==typeof e?e:r,o[1]=l;for(var d=2;d<i;d++)o[d]=t[d];return a.createElement.apply(null,o)}return a.createElement.apply(null,t)}u.displayName="MDXCreateElement"},3660:function(e,n,t){t.r(n),t.d(n,{assets:function(){return s},contentTitle:function(){return p},default:function(){return m},frontMatter:function(){return l},metadata:function(){return d},toc:function(){return c}});var a=t(7462),r=t(3366),i=(t(7294),t(3905)),o=["components"],l={sidebar_position:2},p="Events",d={unversionedId:"api/events",id:"api/events",title:"Events",description:"Events are a core concept in Auto Traffic Control. Every change in the game",source:"@site/docs/api/events.md",sourceDirName:"api",slug:"/api/events",permalink:"/docs/api/events",draft:!1,editUrl:"https://github.com/jdno/auto-traffic-control/tree/main/docs/docs/api/events.md",tags:[],version:"current",sidebarPosition:2,frontMatter:{sidebar_position:2},sidebar:"api",previous:{title:"Introduction",permalink:"/docs/api/"},next:{title:"Types",permalink:"/docs/api/types"}},s={},c=[{value:"<code>AirplaneCollided</code>",id:"airplanecollided",level:2},{value:"<code>AirplaneDetected</code>",id:"airplanedetected",level:2},{value:"<code>AirplaneLanded</code>",id:"airplanelanded",level:2},{value:"<code>AirplaneMoved</code>",id:"airplanemoved",level:2},{value:"<code>FlightPlanUpdated</code>",id:"flightplanupdated",level:2},{value:"<code>LandingAborted</code>",id:"landingaborted",level:2},{value:"<code>GameStarted</code>",id:"gamestarted",level:2},{value:"<code>GameStopped</code>",id:"gamestopped",level:2}],u={toc:c};function m(e){var n=e.components,t=(0,r.Z)(e,o);return(0,i.kt)("wrapper",(0,a.Z)({},u,t,{components:n,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"events"},"Events"),(0,i.kt)("p",null,"Events are a core concept in ",(0,i.kt)("strong",{parentName:"p"},"Auto Traffic Control"),". Every change in the game\nis streamed to players as an event. This page lists every available event type\nand its payload."),(0,i.kt)("h2",{id:"airplanecollided"},(0,i.kt)("inlineCode",{parentName:"h2"},"AirplaneCollided")),(0,i.kt)("p",null,"When two airplanes get too close together, this event is triggered. The event\npayload contains the ids of the two airplanes."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message AirplaneCollided {\n  string id1 = 1;\n  string id2 = 2;\n}\n")),(0,i.kt)("h2",{id:"airplanedetected"},(0,i.kt)("inlineCode",{parentName:"h2"},"AirplaneDetected")),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"AirplaneDetected")," event is triggered every time that a new airplane is\nspawned. All available information about the new airplane is included in the\n",(0,i.kt)("a",{parentName:"p",href:"/docs/api/types#airplane"},(0,i.kt)("inlineCode",{parentName:"a"},"Airplane"))," message type in the event payload."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message AirplaneDetected {\n  Airplane airplane = 1;\n}\n")),(0,i.kt)("h2",{id:"airplanelanded"},(0,i.kt)("inlineCode",{parentName:"h2"},"AirplaneLanded")),(0,i.kt)("p",null,"When an airplane lands, the ",(0,i.kt)("inlineCode",{parentName:"p"},"AirplaneLanded")," event is triggered. The event\npayload contains the ",(0,i.kt)("inlineCode",{parentName:"p"},"id")," of the airplane."),(0,i.kt)("div",{className:"admonition admonition-caution alert alert--warning"},(0,i.kt)("div",{parentName:"div",className:"admonition-heading"},(0,i.kt)("h5",{parentName:"div"},(0,i.kt)("span",{parentName:"h5",className:"admonition-icon"},(0,i.kt)("svg",{parentName:"span",xmlns:"http://www.w3.org/2000/svg",width:"16",height:"16",viewBox:"0 0 16 16"},(0,i.kt)("path",{parentName:"svg",fillRule:"evenodd",d:"M8.893 1.5c-.183-.31-.52-.5-.887-.5s-.703.19-.886.5L.138 13.499a.98.98 0 0 0 0 1.001c.193.31.53.501.886.501h13.964c.367 0 .704-.19.877-.5a1.03 1.03 0 0 0 .01-1.002L8.893 1.5zm.133 11.497H6.987v-2.003h2.039v2.003zm0-3.004H6.987V5.987h2.039v4.006z"}))),"caution")),(0,i.kt)("div",{parentName:"div",className:"admonition-content"},(0,i.kt)("p",{parentName:"div"},"After landing, the airplane is despawned and cannot be queried anymore."))),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message AirplaneLanded {\n  string id = 1;\n}\n")),(0,i.kt)("h2",{id:"airplanemoved"},(0,i.kt)("inlineCode",{parentName:"h2"},"AirplaneMoved")),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"AirplaneMoved")," event is triggered whenever an airplane moves on the map.\nSince airplanes cannot stand still, this event gets send continuously for every\nairplane in the game."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message AirplaneMoved {\n  string id = 1;\n  Point point = 2;\n}\n")),(0,i.kt)("h2",{id:"flightplanupdated"},(0,i.kt)("inlineCode",{parentName:"h2"},"FlightPlanUpdated")),(0,i.kt)("p",null,"When an airplane gets a new flight plan, the ",(0,i.kt)("inlineCode",{parentName:"p"},"FlightPlanUpdated")," event is sent.\nFlight plans are usually updated by the player. But when an airplane reaches the\nlast node in its flight plane, the game will generate a random flight plan."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message FlightPlanUpdated {\n  string id = 1;\n  repeated Node flight_plan = 2;\n}\n")),(0,i.kt)("h2",{id:"landingaborted"},(0,i.kt)("inlineCode",{parentName:"h2"},"LandingAborted")),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"LandingAborted")," event is sent when an airplane attempts a landing on the\nwrong airport. For example, a red airplane tries to land at the blue airport."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message LandingAborted {\n  string id = 1;\n}\n")),(0,i.kt)("h2",{id:"gamestarted"},(0,i.kt)("inlineCode",{parentName:"h2"},"GameStarted")),(0,i.kt)("p",null,"When a new game is started, the ",(0,i.kt)("inlineCode",{parentName:"p"},"GameStarted")," event is triggered. The event\npayload contains the map that was generated for this game session."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message GameStarted {\n  Map map = 1;\n}\n")),(0,i.kt)("h2",{id:"gamestopped"},(0,i.kt)("inlineCode",{parentName:"h2"},"GameStopped")),(0,i.kt)("p",null,"When the game ends, e.g. because two airplanes got too close together, the\n",(0,i.kt)("inlineCode",{parentName:"p"},"GameStopped")," event is sent. The event payload contains the score that the\nplayer achieved."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message GameStopped {\n  uint32 score = 1;\n}\n")))}m.isMDXComponent=!0}}]);
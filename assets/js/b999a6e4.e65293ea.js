"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[927],{3905:function(e,t,n){n.d(t,{Zo:function(){return d},kt:function(){return m}});var a=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function p(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var l=a.createContext({}),s=function(e){var t=a.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},d=function(e){var t=s(e.components);return a.createElement(l.Provider,{value:t},e.children)},c={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},u=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,l=e.parentName,d=p(e,["components","mdxType","originalType","parentName"]),u=s(n),m=r,f=u["".concat(l,".").concat(m)]||u[m]||c[m]||i;return n?a.createElement(f,o(o({ref:t},d),{},{components:n})):a.createElement(f,o({ref:t},d))}));function m(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=u;var p={};for(var l in t)hasOwnProperty.call(t,l)&&(p[l]=t[l]);p.originalType=e,p.mdxType="string"==typeof e?e:r,o[1]=p;for(var s=2;s<i;s++)o[s]=n[s];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}u.displayName="MDXCreateElement"},4902:function(e,t,n){n.r(t),n.d(t,{assets:function(){return d},contentTitle:function(){return l},default:function(){return m},frontMatter:function(){return p},metadata:function(){return s},toc:function(){return c}});var a=n(7462),r=n(3366),i=(n(7294),n(3905)),o=["components"],p={sidebar_position:3},l="Types",s={unversionedId:"api/types",id:"api/types",title:"Types",description:"The API defines a set of message types for the resources in the game, which are",source:"@site/docs/api/types.md",sourceDirName:"api",slug:"/api/types",permalink:"/docs/api/types",draft:!1,editUrl:"https://github.com/jdno/auto-traffic-control/tree/main/docs/docs/api/types.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"api",previous:{title:"Events",permalink:"/docs/api/events"},next:{title:"Airplane",permalink:"/docs/api/Services/airplane-service"}},d={},c=[{value:"Airplane",id:"airplane",level:2},{value:"Airport",id:"airport",level:2},{value:"Map",id:"map",level:2},{value:"Node",id:"node",level:2},{value:"Point",id:"point",level:2},{value:"Tag",id:"tag",level:2},{value:"Version",id:"version",level:2}],u={toc:c};function m(e){var t=e.components,n=(0,r.Z)(e,o);return(0,i.kt)("wrapper",(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h1",{id:"types"},"Types"),(0,i.kt)("p",null,"The API defines a set of message types for the resources in the game, which are\nused in requests and responses."),(0,i.kt)("h2",{id:"airplane"},"Airplane"),(0,i.kt)("p",null,"Airplanes in Auto Traffic Control are uniquely identified by their ",(0,i.kt)("inlineCode",{parentName:"p"},"id"),". The\n",(0,i.kt)("inlineCode",{parentName:"p"},"id")," is passed as an argument to commands that can alter the state of an\nairplane, e.g. update its flight plan."),(0,i.kt)("p",null,"The position of an airplane is provided as a ",(0,i.kt)("a",{parentName:"p",href:"/docs/api/types#point"},(0,i.kt)("inlineCode",{parentName:"a"},"Point")),"\ncoordinate on the map."),(0,i.kt)("p",null,"The flight plan of an airplane is represented by a list of nodes. It can be\nupdated through the ",(0,i.kt)("inlineCode",{parentName:"p"},"AirplaneService"),"."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Airplane {\n  string id = 1;\n  Point point = 2;\n  repeated Node flight_plan = 3;\n  Tag tag = 4;\n}\n")),(0,i.kt)("h2",{id:"airport"},"Airport"),(0,i.kt)("p",null,"Airports have a location and a color."),(0,i.kt)("p",null,"The location of an airport is provided as a ",(0,i.kt)("a",{parentName:"p",href:"/docs/api/types#node"},(0,i.kt)("inlineCode",{parentName:"a"},"Node")),"\ncoordinate, since it must be reachable via the routing grid."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Airport {\n  Node node = 1;\n  Tag tag = 2;\n}\n")),(0,i.kt)("h2",{id:"map"},"Map"),(0,i.kt)("p",null,"The map type provides a full representation of the map that is used in the\ncurrent game session."),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"The airports on the map are provided as a list of\n",(0,i.kt)("a",{parentName:"li",href:"/docs/api/types#airport"},(0,i.kt)("inlineCode",{parentName:"a"},"Airport"))," types."),(0,i.kt)("li",{parentName:"ul"},"The routing grid consists of a list of ",(0,i.kt)("a",{parentName:"li",href:"/docs/api/types#node"},(0,i.kt)("inlineCode",{parentName:"a"},"Node"))," types.")),(0,i.kt)("p",null,"The routing grid can be indexed using the ",(0,i.kt)("inlineCode",{parentName:"p"},"width")," and ",(0,i.kt)("inlineCode",{parentName:"p"},"height")," of the map."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Map {\n  repeated Airport airports = 1;\n  repeated Node routing_grid = 2;\n  uint32 width = 3;\n  uint32 height = 4;\n}\n")),(0,i.kt)("h2",{id:"node"},"Node"),(0,i.kt)("p",null,"Nodes represent locations on the routing grid."),(0,i.kt)("p",null,"Airplanes cannot move freely across the map, and instead must follow the routing\ngrid. Each reachable point in the routing grid is represented by a ",(0,i.kt)("inlineCode",{parentName:"p"},"Node"),"."),(0,i.kt)("p",null,"Nodes can be ",(0,i.kt)("inlineCode",{parentName:"p"},"restricted"),", which means that airplanes are not allowed to pass\nthrough it. Flight plans containing restricted nodes will be rejected by the\ngame."),(0,i.kt)("p",null,"The location of a node can be converted to a point on the map by calling the\n",(0,i.kt)("inlineCode",{parentName:"p"},"NodeToPoint")," endpoint of the ",(0,i.kt)("a",{parentName:"p",href:"/docs/api/Services/map-service"},(0,i.kt)("inlineCode",{parentName:"a"},"MapService")),"."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Node {\n  int32 longitude = 1;\n  int32 latitude = 2;\n  bool restricted = 3;\n}\n")),(0,i.kt)("h2",{id:"point"},"Point"),(0,i.kt)("p",null,"A location on the map."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Point {\n  int32 x = 1;\n  int32 y = 2;\n}\n")),(0,i.kt)("h2",{id:"tag"},"Tag"),(0,i.kt)("p",null,"Tags match airplanes and airports, and are represented by a color."),(0,i.kt)("p",null,"Every airplane and airport has a tag. The tag of an airplane must match the tag\nof an airport for it to land there."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"enum Tag {\n  TAG_UNSPECIFIED = 0;\n  TAG_BLUE = 1;\n  TAG_RED = 2;\n}\n")),(0,i.kt)("h2",{id:"version"},"Version"),(0,i.kt)("p",null,"The ",(0,i.kt)("inlineCode",{parentName:"p"},"AtcService")," provides an endpoint that can be used to query the version of\nthe game. This can be useful to determine if the player's program is compatible\nwith the game."),(0,i.kt)("p",null,(0,i.kt)("strong",{parentName:"p"},"Auto Traffic Control")," follows ",(0,i.kt)("a",{parentName:"p",href:"https://semver.org/"},"semantic versioning"),", and\nthe version is returned as the combination of a ",(0,i.kt)("inlineCode",{parentName:"p"},"major"),", ",(0,i.kt)("inlineCode",{parentName:"p"},"minor"),", and ",(0,i.kt)("inlineCode",{parentName:"p"},"patch"),"\nversion. Pre-releases might also feature a ",(0,i.kt)("inlineCode",{parentName:"p"},"pre")," label."),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-protobuf"},"message Version {\n  uint64 major = 1;\n  uint64 minor = 2;\n  uint64 patch = 3;\n  string pre = 4;\n}\n")))}m.isMDXComponent=!0}}]);
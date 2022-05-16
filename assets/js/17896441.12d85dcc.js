"use strict";(self.webpackChunkauto_traffic_control=self.webpackChunkauto_traffic_control||[]).push([[918],{7634:function(e,t,a){a.r(t),a.d(t,{default:function(){return pe}});var n=a(7294),l=a(6010),r=a(7462),s=a(5999),i=a(9960);function c(e){var t=e.permalink,a=e.title,r=e.subLabel,s=e.isNext;return n.createElement(i.Z,{className:(0,l.Z)("pagination-nav__link",s?"pagination-nav__link--next":"pagination-nav__link--prev"),to:t},r&&n.createElement("div",{className:"pagination-nav__sublabel"},r),n.createElement("div",{className:"pagination-nav__label"},a))}function o(e){var t=e.previous,a=e.next;return n.createElement("nav",{className:"pagination-nav docusaurus-mt-lg","aria-label":(0,s.I)({id:"theme.docs.paginator.navAriaLabel",message:"Docs pages navigation",description:"The ARIA label for the docs pagination"})},t&&n.createElement(c,(0,r.Z)({},t,{subLabel:n.createElement(s.Z,{id:"theme.docs.paginator.previous",description:"The label used to navigate to the previous doc"},"Previous")})),a&&n.createElement(c,(0,r.Z)({},a,{subLabel:n.createElement(s.Z,{id:"theme.docs.paginator.next",description:"The label used to navigate to the next doc"},"Next"),isNext:!0})))}var d=a(2263),m=a(5551),u=a(373),v=a(5281),b=a(4477);var p={unreleased:function(e){var t=e.siteTitle,a=e.versionMetadata;return n.createElement(s.Z,{id:"theme.docs.versions.unreleasedVersionLabel",description:"The label used to tell the user that he's browsing an unreleased doc version",values:{siteTitle:t,versionLabel:n.createElement("b",null,a.label)}},"This is unreleased documentation for {siteTitle} {versionLabel} version.")},unmaintained:function(e){var t=e.siteTitle,a=e.versionMetadata;return n.createElement(s.Z,{id:"theme.docs.versions.unmaintainedVersionLabel",description:"The label used to tell the user that he's browsing an unmaintained doc version",values:{siteTitle:t,versionLabel:n.createElement("b",null,a.label)}},"This is documentation for {siteTitle} {versionLabel}, which is no longer actively maintained.")}};function E(e){var t=p[e.versionMetadata.banner];return n.createElement(t,e)}function h(e){var t=e.versionLabel,a=e.to,l=e.onClick;return n.createElement(s.Z,{id:"theme.docs.versions.latestVersionSuggestionLabel",description:"The label used to tell the user to check the latest version",values:{versionLabel:t,latestVersionLink:n.createElement("b",null,n.createElement(i.Z,{to:a,onClick:l},n.createElement(s.Z,{id:"theme.docs.versions.latestVersionLinkLabel",description:"The label used for the latest version suggestion link label"},"latest version")))}},"For up-to-date documentation, see the {latestVersionLink} ({versionLabel}).")}function g(e){var t,a=e.className,r=e.versionMetadata,s=(0,d.Z)().siteConfig.title,i=(0,m.gA)({failfast:!0}).pluginId,c=(0,u.J)(i).savePreferredVersionName,o=(0,m.Jo)(i),b=o.latestDocSuggestion,p=o.latestVersionSuggestion,g=null!=b?b:(t=p).docs.find((function(e){return e.id===t.mainDocId}));return n.createElement("div",{className:(0,l.Z)(a,v.k.docs.docVersionBanner,"alert alert--warning margin-bottom--md"),role:"alert"},n.createElement("div",null,n.createElement(E,{siteTitle:s,versionMetadata:r})),n.createElement("div",{className:"margin-top--md"},n.createElement(h,{versionLabel:p.label,to:g.path,onClick:function(){return c(p.name)}})))}function f(e){var t=e.className,a=(0,b.E)();return a.banner?n.createElement(g,{className:t,versionMetadata:a}):null}function N(e){var t=e.className,a=(0,b.E)();return a.badge?n.createElement("span",{className:(0,l.Z)(t,v.k.docs.docVersionBadge,"badge badge--secondary")},n.createElement(s.Z,{id:"theme.docs.versionBadge.label",values:{versionLabel:a.label}},"Version: {versionLabel}")):null}function _(e){var t=e.lastUpdatedAt,a=e.formattedLastUpdatedAt;return n.createElement(s.Z,{id:"theme.lastUpdated.atDate",description:"The words used to describe on which date a page has been last updated",values:{date:n.createElement("b",null,n.createElement("time",{dateTime:new Date(1e3*t).toISOString()},a))}}," on {date}")}function Z(e){var t=e.lastUpdatedBy;return n.createElement(s.Z,{id:"theme.lastUpdated.byUser",description:"The words used to describe by who the page has been last updated",values:{user:n.createElement("b",null,t)}}," by {user}")}function L(e){var t=e.lastUpdatedAt,a=e.formattedLastUpdatedAt,l=e.lastUpdatedBy;return n.createElement("span",{className:v.k.common.lastUpdated},n.createElement(s.Z,{id:"theme.lastUpdated.lastUpdatedAtBy",description:"The sentence used to display when a page has been last updated, and by who",values:{atDate:t&&a?n.createElement(_,{lastUpdatedAt:t,formattedLastUpdatedAt:a}):"",byUser:l?n.createElement(Z,{lastUpdatedBy:l}):""}},"Last updated{atDate}{byUser}"),!1)}var k=a(3366),T="iconEdit_dcUD",U=["className"];function w(e){var t=e.className,a=(0,k.Z)(e,U);return n.createElement("svg",(0,r.Z)({fill:"currentColor",height:"20",width:"20",viewBox:"0 0 40 40",className:(0,l.Z)(T,t),"aria-hidden":"true"},a),n.createElement("g",null,n.createElement("path",{d:"m34.5 11.7l-3 3.1-6.3-6.3 3.1-3q0.5-0.5 1.2-0.5t1.1 0.5l3.9 3.9q0.5 0.4 0.5 1.1t-0.5 1.2z m-29.5 17.1l18.4-18.5 6.3 6.3-18.4 18.4h-6.3v-6.2z"})))}function C(e){var t=e.editUrl;return n.createElement("a",{href:t,target:"_blank",rel:"noreferrer noopener",className:v.k.common.editThisPage},n.createElement(w,null),n.createElement(s.Z,{id:"theme.common.editThisPage",description:"The link label to edit the current page"},"Edit this page"))}var y="tag_hD8n",A="tagRegular_D6E_",x="tagWithCount_i0QQ";function B(e){var t=e.permalink,a=e.label,r=e.count;return n.createElement(i.Z,{href:t,className:(0,l.Z)(y,r?x:A)},a,r&&n.createElement("span",null,r))}var I="tags_XVD_",M="tag_JSN8";function D(e){var t=e.tags;return n.createElement(n.Fragment,null,n.createElement("b",null,n.createElement(s.Z,{id:"theme.tags.tagsListLabel",description:"The label alongside a tag list"},"Tags:")),n.createElement("ul",{className:(0,l.Z)(I,"padding--none","margin-left--sm")},t.map((function(e){var t=e.label,a=e.permalink;return n.createElement("li",{key:a,className:M},n.createElement(B,{label:t,permalink:a}))}))))}var H="lastUpdated_foO9";function V(e){return n.createElement("div",{className:(0,l.Z)(v.k.docs.docFooterTagsRow,"row margin-bottom--sm")},n.createElement("div",{className:"col"},n.createElement(D,e)))}function P(e){var t=e.editUrl,a=e.lastUpdatedAt,r=e.lastUpdatedBy,s=e.formattedLastUpdatedAt;return n.createElement("div",{className:(0,l.Z)(v.k.docs.docFooterEditMetaRow,"row")},n.createElement("div",{className:"col"},t&&n.createElement(C,{editUrl:t})),n.createElement("div",{className:(0,l.Z)("col",H)},(a||r)&&n.createElement(L,{lastUpdatedAt:a,formattedLastUpdatedAt:s,lastUpdatedBy:r})))}function S(e){var t=e.content.metadata,a=t.editUrl,r=t.lastUpdatedAt,s=t.formattedLastUpdatedAt,i=t.lastUpdatedBy,c=t.tags,o=c.length>0,d=!!(a||r||i);return o||d?n.createElement("footer",{className:(0,l.Z)(v.k.docs.docFooter,"docusaurus-mt-lg")},o&&n.createElement(V,{tags:c}),d&&n.createElement(P,{editUrl:a,lastUpdatedAt:r,lastUpdatedBy:i,formattedLastUpdatedAt:s})):null}var F=a(1575),R=a(6043),z="tocCollapsible_bZGK",O="tocCollapsibleContent_NNA8",q="tocCollapsibleExpanded_IqtF",X=a(721),J="tocCollapsibleButton_l22C",G="tocCollapsibleButtonExpanded_KeTX",K=["collapsed"];function Q(e){var t=e.collapsed,a=(0,k.Z)(e,K);return n.createElement("button",(0,r.Z)({type:"button"},a,{className:(0,l.Z)("clean-btn",J,!t&&G,a.className)}),n.createElement(s.Z,{id:"theme.TOCCollapsible.toggleButtonLabel",description:"The label used by the button on the collapsible TOC component"},"On this page"))}function W(e){var t=e.toc,a=e.className,r=e.minHeadingLevel,s=e.maxHeadingLevel,i=(0,R.u)({initialState:!0}),c=i.collapsed,o=i.toggleCollapsed;return n.createElement("div",{className:(0,l.Z)(z,!c&&q,a)},n.createElement(Q,{collapsed:c,onClick:o}),n.createElement(R.z,{lazy:!0,className:O,collapsed:c},n.createElement(X.Z,{toc:t,minHeadingLevel:r,maxHeadingLevel:s})))}var j=a(9649),Y="docItemContainer_vinB",$="docItemCol_DM6M",ee="tocMobile_TmEX",te=a(1944),ae=a(7524),ne=a(8425),le=a(8596),re={breadcrumbsContainer:"breadcrumbsContainer_Xlws",breadcrumbHomeIcon:"breadcrumbHomeIcon_kU5B"},se=a(4996);function ie(e){return n.createElement("svg",(0,r.Z)({viewBox:"0 0 24 24"},e),n.createElement("path",{d:"M10 19v-5h4v5c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-7h1.7c.46 0 .68-.57.33-.87L12.67 3.6c-.38-.34-.96-.34-1.34 0l-8.36 7.53c-.34.3-.13.87.33.87H5v7c0 .55.45 1 1 1h3c.55 0 1-.45 1-1z",fill:"currentColor"}))}function ce(e){var t=e.children,a=e.href,l="breadcrumbs__link";return e.isLast?n.createElement("span",{className:l,itemProp:"name"},t):a?n.createElement(i.Z,{className:l,href:a,itemProp:"item"},n.createElement("span",{itemProp:"name"},t)):n.createElement("span",{className:l},t)}function oe(e){var t=e.children,a=e.active,s=e.index,i=e.addMicrodata;return n.createElement("li",(0,r.Z)({},i&&{itemScope:!0,itemProp:"itemListElement",itemType:"https://schema.org/ListItem"},{className:(0,l.Z)("breadcrumbs__item",{"breadcrumbs__item--active":a})}),t,n.createElement("meta",{itemProp:"position",content:String(s+1)}))}function de(){var e=(0,se.Z)("/");return n.createElement("li",{className:"breadcrumbs__item"},n.createElement(i.Z,{"aria-label":(0,s.I)({id:"theme.docs.breadcrumbs.home",message:"Home page",description:"The ARIA label for the home page in the breadcrumbs"}),className:(0,l.Z)("breadcrumbs__link",re.breadcrumbsItemLink),href:e},n.createElement(ie,{className:re.breadcrumbHomeIcon})))}function me(){var e=(0,ne.s1)(),t=(0,le.Ns)();return e?n.createElement("nav",{className:(0,l.Z)(v.k.docs.docBreadcrumbs,re.breadcrumbsContainer),"aria-label":(0,s.I)({id:"theme.docs.breadcrumbs.navAriaLabel",message:"Breadcrumbs",description:"The ARIA label for the breadcrumbs"})},n.createElement("ul",{className:"breadcrumbs",itemScope:!0,itemType:"https://schema.org/BreadcrumbList"},t&&n.createElement(de,null),e.map((function(t,a){var l=a===e.length-1;return n.createElement(oe,{key:a,active:l,index:a,addMicrodata:!!t.href},n.createElement(ce,{href:t.href,isLast:l},t.label))})))):null}var ue=a(5290);function ve(e){var t,a=e.content,l=a.metadata,r=a.frontMatter,s=a.assets,i=r.keywords,c=l.description,o=l.title,d=null!=(t=s.image)?t:r.image;return n.createElement(te.d,{title:o,description:c,keywords:i,image:d})}function be(e){var t=e.content,a=t.metadata,r=t.frontMatter,s=r.hide_title,i=r.hide_table_of_contents,c=r.toc_min_heading_level,d=r.toc_max_heading_level,m=a.title,u=!s&&void 0===t.contentTitle,b=(0,ae.i)(),p=!i&&t.toc&&t.toc.length>0,E=p&&("desktop"===b||"ssr"===b);return n.createElement("div",{className:"row"},n.createElement("div",{className:(0,l.Z)("col",!i&&$)},n.createElement(f,null),n.createElement("div",{className:Y},n.createElement("article",null,n.createElement(me,null),n.createElement(N,null),p&&n.createElement(W,{toc:t.toc,minHeadingLevel:c,maxHeadingLevel:d,className:(0,l.Z)(v.k.docs.docTocMobile,ee)}),n.createElement("div",{className:(0,l.Z)(v.k.docs.docMarkdown,"markdown")},u&&n.createElement("header",null,n.createElement(j.Z,{as:"h1"},m)),n.createElement(ue.Z,null,n.createElement(t,null))),n.createElement(S,e)),n.createElement(o,{previous:a.previous,next:a.next}))),E&&n.createElement("div",{className:"col col--3"},n.createElement(F.Z,{toc:t.toc,minHeadingLevel:c,maxHeadingLevel:d,className:v.k.docs.docTocDesktop})))}function pe(e){var t="docs-doc-id-"+e.content.metadata.unversionedId;return n.createElement(te.FG,{className:t},n.createElement(ve,e),n.createElement(be,e))}},4477:function(e,t,a){a.d(t,{E:function(){return i},q:function(){return s}});var n=a(7294),l=a(9688),r=n.createContext(null);function s(e){var t=e.children,a=e.version;return n.createElement(r.Provider,{value:a},t)}function i(){var e=(0,n.useContext)(r);if(null===e)throw new l.i6("DocsVersionProvider");return e}}}]);
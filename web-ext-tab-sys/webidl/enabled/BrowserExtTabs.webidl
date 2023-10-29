enum TabMutedReasonDetails { "user", "capture", "extension" };
enum RunAt { "document_end", "document_idle", "document_start" };
enum TabStatus { "complete", "loading" };
enum WindowTypes { "app", "normal", "panel", "popup" };
enum TabsCaptureVisibleTabFormat { "jpeg", "png" };
dictionary Tab {
    boolean active;
    boolean audible;
    DOMString? favIconUrl;
    boolean highlighted;
    long height;
    long id;
    boolean incognito;
    long index;
    TabMutedDetails? mutedDetails;
    long openerTabId;
    boolean pinned;
    DOMString? sessionId;
    DOMString? status;
    DOMString? title;
    DOMString? url;
    long width;
    long windowId;
};
dictionary TabMutedDetails {
    DOMString? extensionId;
    boolean muted;
    TabMutedReasonDetails? reason;
};
dictionary TabConnectDetails {
    long frameId;
    DOMString? name;
};
dictionary TabCreateDetails {
    boolean active;
    long index;
    long openerTabId;
    boolean pinned;
    DOMString? url;
    long windowId;
};
dictionary TabScriptAndCSSDetails {
    boolean allFrames;
    DOMString? code;
    DOMString? file;
    long frameId;
    boolean matchAboutBlank;
    RunAt? runAt;
};
dictionary TabsCaptureVisibleTabDetails {
    TabsCaptureVisibleTabFormat imageCaptureFormat;
    long jpegQuality;
};
dictionary TabQueryDetails {
    boolean active;
    boolean audible;  
    boolean currentWindow;
    boolean highlighted;
    long index;
    boolean lastFocusedWindow;
    boolean muted;
    boolean pinned;
    TabStatus? status;
    DOMString? title;
    (DOMString? or sequence<DOMString>?) url;
    long windowId;
    WindowType? windowType;
};
dictionary TabReloadDetails {
    boolean bypassCache;
};
dictionary TabSendMessageDetails {
    long frameId;
};
dictionary TabUpdateDetails {
    boolean active;
    boolean highlighted;
    boolean muted;
    long openerTabId;
    boolean pinned;
    DOMString? url;
};
dictionary TabIdWindowId {
    long tabId;
    long windowId;
};
dictionary TabsWindowIdIsWindowClosing {
    boolean isWindowClosing;
    long windowId;
};
dictionary TabsOnUpdatedChangeDetails {
    boolean audible;    
    DOMString? favIconUrl;
    TabMutedDetails? mutedDetails;
    boolean pinned;
    DOMString? status;
    DOMString? title;
    DOMString? url;
};
callback TabsOnActivatedCallback = void (TabIdWindowId activeDetails);
callback TabsOnCreatedCallback = void (TabTab tab);
callback TabsOnRemovedCallback = void (long tabId, TabsWindowIdIsWindowClosing removeDetails);
callback TabsOnUpdatedCallback = void (long tabId, TabsOnUpdatedChangeDetails details, Tab tab);
[NoInterfaceObject]
interface Tabs {
    Promise<DOMString> captureVisibleTab(long windowId, TabsCaptureVisibleTabDetails details);
    RuntimePort connect(long tabId, TabConnectDetails details);
    Promise<Tab> create(TabCreateDetails details);
    void executeScript(long tabId, TabScriptAndCSSDetails details);
    Promise<Tab> get(long tabId);
    Promise<Tab> getCurrent();
    void insertCSS(long tabId, TabScriptAndCSSDetails details);
    void onActivated(TabsOnActivatedCallback callback);
    void onCreated(TabsOnCreatedCallback callback);
    void onRemoved(TabsOnRemovedCallback callback);
    void onUpdated(TabsOnUpdatedCallback callback);
    Promise<Tab> query(TabQueryDetails queryDetails);
    Promise<void> reload(TabReloadDetails details);
    Promise<void> remove((long or sequence<long>) tabIds);
    Promise<any> sendMessage(long tabId, any message, optional TabSendMessageOptions details);
    Promise<Tab> update(long tabId, TabUpdateDetails details);
};
[NoInterfaceObject, Exposed=Window, CheckAnyPermissions="browserExtTabs"]
interface TabsAPI {
readonly attribute Tabs tabs; 
};
Browser implements TabsAPI;

Refactor "Steps" into Handlers (keep the "Steps" name)
A step is either sync or async  

Steps takes Reactions as input 
Steps are modular (you can add them to your UI or not)
-> Providing core logic with cleanly-separated behaviour and ordering 
    * Layout
    * Drawing
    * Event handling 
-> Providing optional features such as 
    * Futures polling
    * Winit handling 
    * Process spawning 
    * ... ? 

Split into utopia_sync and utopia_async
-> AsyncRuntime  -> Sink<B::Event>
-> SyncRuntime   -> Queue<B::Event>

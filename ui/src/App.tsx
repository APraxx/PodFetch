import './App.css'
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {SideBar} from "./components/SideBar";
import {Header} from "./components/Header";
import {useAppSelector} from "./store/hooks";
import {Podcasts} from "./pages/Podcasts";
import {PodcastDetailPage} from "./pages/PodcastDetailPage";
import {Modal} from "./components/Modal";
import {AudioPlayer} from "./components/AudioPlayer";
import {Homepage} from "./pages/Homepage";

const App = ()=> {
    const sideBarCollapsed = useAppSelector(state=>state.common.sideBarCollapsed)
    const currentPodcast = useAppSelector(state=>state.audioPlayer.currentPodcastEpisode)
    return (
      <BrowserRouter basename="/ui">
          <div className="grid  grid-rows-[auto_1fr] h-full md:grid-cols-[300px_1fr]">
              <Header/>
              <SideBar/>
              <div className={`col-span-6 md:col-span-5 ${sideBarCollapsed?'xs:col-span-5':'hidden'} md:block w-full overflow-x-auto`}>
                  <div className="grid grid-rows-[1fr_auto] h-full ">
                  <Routes>
                      <Route path={"/home"} element={<Homepage/>}/>
                      <Route path={"/podcasts"} element={<Podcasts/>}/>
                      <Route path={"/podcasts/:id"} element={<PodcastDetailPage/>}/>
                  </Routes>
                      {currentPodcast&& <AudioPlayer/>}
                  </div>
              </div>
          </div>
      </BrowserRouter>
  )
}

export default App
import {useState} from "react";
import {apiURL} from "../utils/Utilities";
import {SubmitHandler, useForm} from "react-hook-form";
import axios, {AxiosError} from "axios";
import {useTranslation} from "react-i18next";
import {useNavigate} from "react-router-dom";
import {useAppDispatch, useAppSelector} from "../store/hooks";
import {setLoginData} from "../store/CommonSlice";
import {OIDCLogin} from "./OIDCButton";
import {Loading} from "./Loading";

export type LoginData = {
    username: string,
    password: string,
    rememberMe: string
};

export const LoginComponent = () => {
    const dispatch = useAppDispatch()
    const {register, handleSubmit, formState: {}} = useForm<LoginData>();
    const [alert, setAlert] = useState<string>()
    const {t} = useTranslation()
    const navigate = useNavigate()
    const configModel = useAppSelector(state => state.common.configModel)


    const onSubmit: SubmitHandler<LoginData> = (data, p) => {
        p?.preventDefault()
        axios.post(apiURL + "/login", data)
            .then(() => {
                const basicAuthString = btoa(data.username + ":" + data.password)
                if (data.rememberMe){
                    localStorage.setItem("auth", basicAuthString)
                }
                else{
                    sessionStorage.setItem("auth", basicAuthString)
                }
                dispatch(setLoginData(data))
                axios.defaults.headers.common['Authorization'] = 'Basic ' + basicAuthString;
                setTimeout(()=>navigate('/'), 100)

            })
            .catch((e: AxiosError) => {
               setAlert(e.response!.data as string)
            })
    }

    if (!configModel){
        return <Loading/>
    }
    return <section className="bg-gray-900 h-full">
        <div className="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
            <a href="#" className="flex items-center mb-6 text-2xl font-semibold text-white">
                <i className="fa-solid fa-music mr-5"></i>
                PodFetch
            </a>
            <div
                className="w-full rounded-lg shadow border md:mt-0 sm:max-w-md xl:p-0 bg-gray-800 border-gray-700">
                <div className="p-6 space-y-4 md:space-y-6 sm:p-8">
                    <h1 className="text-xl font-bold leading-tight tracking-tight md:text-2xl text-white">
                        {t('sign-in-to-podfetch')}
                    </h1>
                    {configModel?.basicAuth&&<form className="space-y-4 md:space-y-6" onSubmit={handleSubmit(onSubmit)}>
                        {alert&&<div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative"
                             role="alert">
                                                <strong className="font-bold">{t('error-authenticating')}</strong>
                                                <br/>
                                                <span className="block sm:inline">{alert}</span>
                                                <span className="absolute top-0 bottom-0 right-0 px-4 py-3">
                             <svg className="fill-current h-6 w-6 text-red-500" role="button" xmlns="http://www.w3.org/2000/svg"
                             viewBox="0 0 20 20"><title>Close</title><path
                            d="M14.348 14.849a1.2 1.2 0 0 1-1.697 0L10 11.819l-2.651 3.029a1.2 1.2 0 1 1-1.697-1.697l2.758-3.15-2.759-3.152a1.2 1.2 0 1 1 1.697-1.697L10 8.183l2.651-3.031a1.2 1.2 0 1 1 1.697 1.697l-2.758 3.152 2.758 3.15a1.2 1.2 0 0 1 0 1.698z"/></svg>
                            </span>
                        </div>}
                        <div>
                            <label htmlFor="email"
                                   className="block mb-2 text-sm font-medium text-white">{t('username')!}</label>
                            <input type="username" {...register('username', {required: true})} id="username"
                                   autoComplete="username"
                                   className="border sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
                                   placeholder={t('your-username')!}/>
                        </div>
                        <div>
                            <label htmlFor="password"
                                   className="block mb-2 text-sm font-medium  text-white">{t('password')}</label>
                            <input type="password" id="password" autoComplete="current-password"
                                   placeholder="••••••••" {...register('password', {required: true})}
                                   className="border sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"/>
                        </div>
                        <div className="flex items-center justify-between">
                            <div className="flex items-start">
                                <div className="flex items-center h-5">
                                    <input id="remember" aria-describedby="remember"
                                           type="checkbox" {...register('rememberMe')}
                                           className="w-4 h-4 border  rounded focus:ring-3 focus:ring-primary-300 bg-gray-700 border-gray-600 focus:ring-primary-600 ring-offset-gray-800"/>
                                </div>
                                <div className="ml-3 text-sm">
                                    <label htmlFor="remember" className="text-gray-300">{t('remember-me')}</label>
                                </div>
                            </div>
                        </div>
                        <button type="submit"
                                className="w-full text-white bg-primary-600 hover:bg-primary-700 focus:ring-4 focus:outline-none focus:ring-primary-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center bg-blue-800 bg-primary-700 hover:bg-blue-700 focus:ring-primary-800">{t('sign-in')}</button>
                    </form>}
                    {configModel.oidcConfigured&& configModel.oidcConfig&&
                            <OIDCLogin/>
                    }
                </div>
            </div>
        </div>
    </section>
}

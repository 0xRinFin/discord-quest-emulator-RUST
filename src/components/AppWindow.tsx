import { PropsWithChildren } from "react"
import "../style/App.css";

const AppWindow = (props: PropsWithChildren) => {
    return (
        <main className="items-center justify-center flex-col flex text-center h-screen">
            {props.children}
        </main>
    )
}

export default AppWindow
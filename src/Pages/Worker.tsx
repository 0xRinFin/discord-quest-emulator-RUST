import AppWindow from "../components/AppWindow"

const WorkerPage = (props: {workr: any}) => {

    return (
        <AppWindow>
            <h1 className="text-center font-bold text-3xl">{props.workr}</h1>
        </AppWindow>
    )
};

export default WorkerPage;
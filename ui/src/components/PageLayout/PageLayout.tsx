import s from './PageLayout.module.css'
import Dashboard from '../dashboard/Dashboard'

interface Props {
    children: JSX.Element
}

function PageLayout({ children }: Props) {
    return (
        <div className={s.wrapper}>
            <Dashboard />
            {/* <MobileNavBar /> */}
            {/* <PageFooter /> */}
        </div>
    )
}
export default PageLayout

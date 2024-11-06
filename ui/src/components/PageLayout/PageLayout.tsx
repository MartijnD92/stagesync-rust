import s from './PageLayout.module.css'
import SideBar from '../SideBar'

interface Props {
    children: JSX.Element
}

function PageLayout({ children }: Props) {
    return (
        <div className={s.wrapper}>
            <SideBar componentClass={s.sidebar} />
            {/* <MobileNavBar /> */}
            <div className={s.maincontent}>{children}</div>
            {/* <PageFooter /> */}
        </div>
    )
}
export default PageLayout

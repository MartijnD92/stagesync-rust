import s from './PageLayout.module.css'

interface Props {
    children: JSX.Element
}

function PageLayout({ children }: Props) {
    return (
        <div className={s.wrapper}>
            {/* <MobileNavBar /> */}
            {/* <PageFooter /> */}
            {children}
        </div>
    )
}
export default PageLayout

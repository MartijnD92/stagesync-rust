interface Props {
    children: JSX.Element
}

function PageLayout({ children }: Props) {
    return (
        <div className="page-layout">
            {/* <NavBar />
      <MobileNavBar /> */}
            <div className="page-layout__content">{children}</div>
            {/* <PageFooter /> */}
        </div>
    )
}

export default PageLayout

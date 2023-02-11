function clear() {
    window.scrollTo(0, 0);
    document.getElementsByTagName('html')[0].style.visibility = 'hidden';
    document.getElementsByClassName('lf_area')[0].style.visibility = 'visible';
    document.getElementsByTagName('header')[0].style.display = 'none';
    document.getElementsByClassName('contentPadding')[0].style.padding = '10px';
}

document.onreadystatechange = function () {
    if (document.readyState == 'interactive') {
        clear();
    }
}
function clear() {
    window.scrollTo(0, 0);
    document.getElementsByTagName('body')[0].style.margin = '0';
    document.getElementsByTagName('header')[0].style.display = 'none';
    document.getElementsByTagName('footer')[0].style.display = 'none';
    document.getElementsByClassName('search')[0].style.display = 'none';
    document.querySelectorAll('div .g_pr_ad_network')[1].style.display = 'none';
    document.querySelectorAll('div .g_pr_ad_network')[3].style.margin = '0';
    Array.from(document.querySelectorAll('ins')).forEach(e => {
        e.style.display = 'none'
    });
    Array.from(document.querySelectorAll('iframe:not(#player)')).forEach(e => {
        e.style.display = 'none'
    });
}

document.onreadystatechange = function () {
    if (document.readyState == 'interactive') {
        clear();
    }
}
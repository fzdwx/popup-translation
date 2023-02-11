function clear() {
    window.scrollTo(0, 0);
    document.getElementsByTagName('html')[0].style.visibility = 'hidden';
    document.getElementsByClassName('main')[0].style.visibility = 'visible';
    document.getElementsByClassName('main')[0].style.margin = '0';
    document.getElementById('dshared').style.display = 'none';
    document.getElementById('aswift_0_host').style.display = 'none';
    document.getElementById('aswift_1_host').style.display = 'none';
    document.getElementById('aswift_2_host').style.display = 'none';
    document.getElementById('aswift_3_host').style.display = 'none';
    document.getElementById('content').style.padding = '0';
    document.getElementById('content').style.margin = '0';
    document.getElementById('footer').style.display = 'none';
    document.getElementsByClassName('copyright')[0].style.display = 'none';
    Array.from(document.querySelectorAll('iframe')).forEach(e => {
        e.style.display = 'none'
    })
}

document.onreadystatechange = function () {
    if (document.readyState == 'interactive') {
        clear();
    }
}
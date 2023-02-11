function clear() {
    window.scrollTo(0, 0);
    document.getElementsByTagName('html')[0].style.visibility = 'hidden';
    document.getElementById('results').style.visibility = 'visible';
    document.getElementById('scontainer').style.margin = '0';
    document.getElementById('scontainer').style.padding = '0';
    document.getElementById('result_navigator').style.display = 'none';
    document.getElementById('container').style.padding = '0';
    document.getElementById('container').style.paddingLeft = '10px';
    document.getElementById('container').style.margin = '0';
    document.getElementById('topImgAd').style.display = 'none';
}

document.onreadystatechange = function () {
    if (document.readyState == 'interactive') {
        clear();
    }
}
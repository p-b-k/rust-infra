//////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Services page support
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

function show_service_panel(elem, key) {
  console.log("show_service_panel(" + elem + ', ' + key + ")");
  let panel = document.getElementById("service-panel");
  if (panel.style.visibility == 'hidden') {
    panel.style.visibility = 'visible';
  } else if (panel.style.visibility == 'visible') {
    panel.style.visibility = 'hidden';
  } else {
    console.log('panel.style.visibility = ' + panel.style.visibility);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Core Table Component Functionality
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

function addColumnToHead (tr, col) {
  console.log('addColumnToHead: called on ' + tr + ', ' + col);
  let td = document.createElement('td');
  let span = document.createElement('span');
  td.appendChild(span)
  tr.appendChild(td)
  
  span.innerText = col.text;
  span.classList.add(col.class);
}

function appendBodyTd (tr, row, col) {
  console.log('addColumnToHead: called on ' + tr + ', ' + col);
  let td = document.createElement('td');
  let span = document.createElement('span');
  td.appendChild(span);
  tr.appendChild(td);
  
  console.log ('value of column ' + col.column + ' is ' + row [col.column]);
  
  span.innerText = row [col.column];
  span.classList.add(col.class);
}

function rePopulateHead (tableId, data) {
  thead = document.getElementById(tableId);
  if (thead) {
    thead.innerHTML = '';

    let headData = data[tableId].head;
    if (headData) {
      let tr = document.createElement('tr');
      thead.appendChild(tr);
      headData.columns.forEach(function (col) { addColumnToHead (tr, col); });
    } else {
      console.log('rePopulateHead: No column data found');
    }
    
  } else {
    console.log('rePopulateHead: thead not found for ' + tableId);
  }
}


function rePopulateBody (tableId, data) {
  tbody = document.getElementById(tableId);
  if (tbody) {
    tbody.innerHTML = '';

    let bodyData = data[tableId].body;
    if (bodyData) {
      let headData = data[tableId].head;
      if (headData) {
        bodyData.forEach(function (row) {
          let tr = document.createElement('tr');
          tbody.appendChild(tr);
          headData.columns.forEach (function (col) {
            appendBodyTd (tr, row, col);
          });
        });
      } else {
        console.log('rePopulateBody: No table column data found for table body');
      }
    } else {
      console.log('rePopulateBody: No table body data found');
    }
    
  } else {
    console.log('rePopulateBody: tbody not found for ' + tableId);
  }
}

function populateTable (tableId, headUrl, bodyUrl, data) {
  let bodyCallback = function (obj) {
    data[tableId].body = obj;

    rePopulateBody (tableId, data);
  }

  let headCallback = function (obj) {
    data[tableId]['head'] = obj;

    let toolBar = document.createElement('table');
    toolBar.classList = [ 'search-bar' ];

    let tr = document.createElement('tr');
    tr.classList = [ 'search-bar' ];

    let title = document.createElement('td');
    title.width = "99%";

    let titleDiv =  document.createElement('div');
    titleDiv.classList = [ 'table-header' ];
    titleDiv.innerText = obj.title;
    title.appendChild(titleDiv);
    tr.append(title);
    
    if (obj.searchUrl) {
      let iconTd = document.createElement('td');
      let img = document.createElement('img');
      img.src = "/static/svg/search.svg";
      img.height = "16";
      img.width = "16";
      iconTd.appendChild(img);
      tr.appendChild(iconTd);

      let inputTd = document.createElement('td');
      let input = document.createElement('input');
      inputTd.appendChild(input);
      tr.appendChild(inputTd);
    }

    {
      let td = document.createElement('td');
      let img = document.createElement('img');
      img.src = "/static/svg/search.svg";
      img.height = "16";
      img.width = "16";
      td.appendChild(img);
      tr.appendChild(td);
    }

    {
      let td = document.createElement('td');
      let img = document.createElement('img');
      img.src = "/static/svg/search.svg";
      img.height = "16";
      img.width = "16";
      td.appendChild(img);
      tr.appendChild(td);
    }

    rePopulateHead (tableId, data);
    processGetRequest(bodyUrl, bodyCallback);
  }

  data[tableId] = {};
  processGetRequest(headUrl, headCallback);
}

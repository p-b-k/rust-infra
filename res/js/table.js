////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Core Table Component Functionality
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

function addColumnToHead (tr, col) {
  console.log('addColumnToHead: called on ' + tr + ', ' + col);
  let td = document.createElement('th');
  if (col.width) {
    td.style = "width: " + col.width + "px;";
  }
  let span = document.createElement('span');
  td.appendChild(span)
  tr.appendChild(td)
  
  span.innerText = col.text;
  span.classList.add(col.class);
}

function appendDOBodyTd (tr, row, col) {
  console.log('appendDOBodyTd: called on ' + tr + ', ' + col);
  let td = document.createElement('td');
  let span = document.createElement('span');
  td.appendChild(span);
  tr.appendChild(td);
  
  console.log ('value of column ' + col.column + ' is ' + row [col.column]);
  
  span.innerText = row [col.column];
  span.classList.add(col.class);
}

function rePopulateToolbar (toolTable, obj) {
  toolTable.classList = [ 'search-bar' ];

  let tr = document.createElement('tr');
  tr.classList = [ 'search-bar' ];
  toolTable.appendChild(tr);

  let title = document.createElement('td');
  title.width = "99%";

  let titleDiv =  document.createElement('div');
  titleDiv.classList = [ 'table-header' ];
  titleDiv.innerText = obj.title;
  title.appendChild(titleDiv);
  tr.append(title);
  
  if (obj.search_url) {
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

  { // Just to scope td and img
    let td = document.createElement('td');
    let img = document.createElement('img');
    img.src = "/static/svg/copy.svg";
    img.height = "16";
    img.width = "16";
    td.appendChild(img);
    tr.appendChild(td);
  }

  if (obj.refresh_url) {
    let td = document.createElement('td');
    let img = document.createElement('img');
    img.src = "/static/svg/reload.svg";
    img.height = "16";
    img.width = "16";
    td.appendChild(img);
    tr.appendChild(td);
  }
}

function rePopulateColumns (thead, tableId) {
  thead.innerHTML = '';

  let headData = data[tableId].head;
  if (headData) {
    console.log('headData = ' + headData + ', headData.columns = ' + headData.columns);
    let tr = document.createElement('tr');
    thead.appendChild(tr);
    headData.columns.forEach(function (col) { addColumnToHead (tr, col); });
  } else {
    console.log('rePopulateColumns: No column data found');
  }
}

function rePopulateDOBody (tbody, tableId) {
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
            appendDOBodyTd (tr, row[1], col);
          });
        });
      } else {
        console.log('rePopulateDOBody: No table column data found for table body');
      }
    } else {
      console.log('rePopulateDOBody: No table body data found');
      let noDataDiv = document.createElement('div');
      noDataDiv.innerText = 'No records found';
      tbody.appendChild(noDataDiv);
    }
    
  } else {
    console.log('rePopulateDOBody: tbody not found for ' + tableId);
  }
}

function rePopulateROBody (tbody, tableId) {
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
            appendDOBodyTd (tr, row, col);
          });
        });
      } else {
        console.log('rePopulateDOBody: No table column data found for table body');
      }
    } else {
      console.log('rePopulateDOBody: No table body data found');
      let noDataDiv = document.createElement('div');
      noDataDiv.innerText = 'No records found';
      tbody.appendChild(noDataDiv);
    }
    
  } else {
    console.log('rePopulateDOBody: tbody not found for ' + tableId);
  }
}

function populateDOTable (tableId, headUrl, bodyUrl) {
  populateTable(tableId, headUrl, bodyUrl, rePopulateDOBody);
}
  
function populateROTable (tableId, headUrl, bodyUrl) {
  populateTable(tableId, headUrl, bodyUrl, rePopulateROBody);
}
  
function populateTable (tableId, headUrl, bodyUrl, bodyDataCallback) {
  // Find the root element ...
  console.log('tableId = ' + tableId);
  let root = document.getElementById(tableId);
  root.classList = [ 'table-control' ];

  // ... or exit
  if (!root) {
    console.log('no element found for table ' + tableId);
    return;
  }

  // Create the tables for the toolbar and the data
  let toolTable = document.createElement('table');
  toolTable.classList = [ 'search-bar' ];
  let dataTable = document.createElement('table');

  root.appendChild(toolTable);
  root.appendChild(dataTable);

  // Create the table components for the data table
  let dataHead = document.createElement('thead');
  dataHead.classList = [ 'column-header' ];
  let dataBody = document.createElement('tbody');
  dataBody.classList = [ 'table-data' ];
  dataTable.appendChild(dataHead);
  dataTable.appendChild(dataBody);

  // Define the body callback function
  let bodyCallback = function (obj) {
    data[tableId].body = obj;

    // rePopulateBody (dataBody, tableId);
    bodyDataCallback (dataBody, tableId);
  }

  // Define the body callback function
  let headCallback = function (obj) {
    data[tableId]['head'] = obj;

    rePopulateToolbar (toolTable, obj);

    rePopulateColumns (dataHead, tableId);
    processGetRequest(bodyUrl, bodyCallback);
  }

  data[tableId] = {};
  processGetRequest(headUrl, headCallback);
}



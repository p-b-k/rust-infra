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

function rePopulateHead (headId, data) {
  thead = document.getElementById(headId);
  if (thead) {
    thead.innerHTML = '';

    let headData = data.head;
    if (headData) {
      let tr = document.createElement('tr');
      thead.appendChild(tr);
      headData.columns.forEach(function (col) { addColumnToHead (tr, col); });
    } else {
      console.log('rePopulateHead: No column data found');
    }
    
  } else {
    console.log('rePopulateHead: thead not found for ' + headId);
  }
}


function rePopulateBody (bodyId, data) {
  tbody = document.getElementById(bodyId);
  if (tbody) {
    tbody.innerHTML = '';

    let bodyData = data.body;
    if (bodyData) {
      let headData = data.head;
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
    console.log('rePopulateBody: tbody not found for ' + bodyId);
  }
}


function populateTable (headId, bodyId, headUrl, bodyUrl, data) {
  let bodyCallback = function (obj) {
    data.body = obj;

    rePopulateBody (bodyId, data);
  }

  let headCallback = function (obj) {
    data.head = obj;

    rePopulateHead (headId, data);
    processGetRequest(bodyUrl, bodyCallback);
  }

  processGetRequest(headUrl, headCallback);
}




// function populateTableHead (id, url) {
//   let callback = function (obj) {
//     console.log('populateTableHead: obj = ' + obj);    
//     let thead = document.getElementById(id);
//     thead.innerText = '';

//     let tr = document.createElement('tr');
//     thead.appendChild(tr);
//     obj.forEach(function (cdef) { addColumnToHead (tr, cdef); })

//     return obj;
//   };

//   console.log('populateTableHead: about to run callback');
//   return processGetRequest(url, callback);
// }

// function populateTableBody (id, url, cols) {
//   const callback = function (obj) {
//     console.log('populateTableBody: obj = ' + obj);    
//     let tbody = document.getElementById(id);
//     console.log('populateTableBody: tbody = ' + tbody);    

//     tbody.innerText = '';

//     obj.forEach (function (row) {
//       let tr = document.createElement('tr');
//       tbody.append(tr);

//       cols.forEach (function (col) { appendBodyTd (tr, row, col); })  })
//   };

//   processGetRequest(url, callback);
// }

// function populateTable (headId, bodyId, headUrl, bodyUrl) {
//   let cols = populateTableHead (headId, headUrl);
//   console.log('populateTable: cols = ' + cols);
//   populateTableBody (bodyId, bodyUrl, cols);
// }


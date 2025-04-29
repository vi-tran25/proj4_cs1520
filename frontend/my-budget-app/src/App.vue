<script setup>
  import { ref } from 'vue'

  let catId = 0
  const isCatForm = ref(false)
  const isPurForm = ref(false)

  const cats = ref([ { id: catId++, name: "Other", budget: null } ])
  const purs = ref([])

  function setup() {
    getCategories()
    getPurchases()
  }

  // AJAX functions-------------------------------
  async function getCategories() {
    console.log("Fetching categories")
    
    try  {
      const response = await fetch(`http://127.0.0.1:5000/categories/`)
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`)
      }

      const result = await response.json()
      cats.value = result
      console.log(result)
    } catch (error) {
      console.log("Error fetching categories")
      console.error(error.message)
    }
  }

  async function getPurchases() {
    console.log("Fetching purchases")

    try  {
      const response = await fetch(`http://127.0.0.1:5000/purchases/`)
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`)
      }

      const result = await response.json()
      purs.value = result
      mapCategoryById()
      checkOverBudget()
      console.log(purs.value)
    } catch (error) {
      console.log("Error fetching purchases")
      console.error(error.message)
    }
  }

  async function sendNewCat() {
    console.log("Adding new category")
    const catName = document.getElementById("categoryName").value
    const catBudStr = document.getElementById("categoryBudget").value
    let catBud = parseInt(catBudStr)

    if (catName == "") {
      console.log("-Error: Empty Name")
      return
    }
    if (catBud <= 0) catBud = null

    try {
      const response = await fetch(`http://127.0.0.1:5000/categories/`, {
        method: "POST",
        headers: { "Content-type": "application/json" },
        body: JSON.stringify({ name: catName, budget: catBud}),
      })
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`)
      }

      const result = await response.json()
      console.log(result)
      console.log("Add new category successfully")
      await getCategories()
      isCatForm.value = false
    } catch (error) {
      console.log(("Error adding new category"))
      console.error(error.message)
    }
  }

  async function sendNewPur() {
    console.log("Adding new purchase")
    const purDesc = document.getElementById("purchaseDesc").value
    const purAmount = parseInt(document.getElementById("purchaseAmount").value)
    const purDate = document.getElementById("purchaseDate").value
    const purCat = document.getElementById("purCatName").value

    if (!validatePur(purAmount, purDate, purCat)) {
      return
    }

    try {
      const response = await fetch(`http://127.0.0.1:5000/purchases/`, {
        method: "POST",
        headers: { "Content-type": "application/json" },
        body: JSON.stringify({ 
          desc: purDesc, 
          amount: purAmount,
          date: purDate,
          category: purCat,
        }),
      })
      if (!response.ok) {
        throw new Error(`Response status: ${response.status}`)
      }

      const result = await response.json()
      console.log(result)
      console.log("Add new category successfully")
      await getPurchases();

      document.getElementById("purchaseDesc").value = "";
      document.getElementById("purchaseAmount").value = "";
      document.getElementById("purchaseDate").value = "";
      document.getElementById("purCatName").value = "";

      isPurForm.value = false
    } catch (error) {
      console.log(("Error adding new category"))
      console.error(error.message)
    }
  }

  // Helper functions----------------------------
  function showCatForm() {
    isCatForm.value = !isCatForm.value
    isPurForm.value = false
  }

  function showPurForm() {
    isCatForm.value = false
    isPurForm.value = !isPurForm.value
  }

  function mapCategoryById() {
    purs.value = purs.value.map(pur => {
      const cat = cats.value.find(cat => cat.id === pur.cat_id)
      return {
        ...pur,
        category: cat ? cat.name : "Unknown"
      }
    })
  }

  function validatePur(amount, date, category) {
    if (!amount || amount < 0) {
      console.log("-Error: Invalid amount")
      return false
    } else if (date == "") {
      console.log("-Error: Invalid date")
      return false
    } else if (category == "") {
      console.log("-Error: Invalid category")
      return false
    }

    return true
  }

  function checkOverBudget() {
    let eachMonthBudget = []

    purs.value.slice().reverse().forEach((pur) => {
      const newDate = pur.date.slice(0, 7)
      const newCat = pur.category
      const cat = cats.value.find(cat => cat.id === pur.cat_id)

      if (cat.budget == null) {
        pur.overBudget = false
        return
      }

      let found = false

      eachMonthBudget.forEach((each) => {
        if (each.date === newDate && each.category === newCat) {
          each.spent += pur.amount
          if (each.spent > each.budget)
            pur.overBudget = true
          
          found = true
        }
        return
      })

      if (!found) {
        let newMonthBudget = { 
          date: newDate, 
          category: newCat, 
          budget: cat.budget, 
          spent: pur.amount
        }
        eachMonthBudget.push(newMonthBudget)
        if (newMonthBudget.spent > newMonthBudget.budget)
            pur.overBudget = true
        else
          pur.overBudget = false
      }
    })
  }

  window.addEventListener("load", setup);
</script>

<!--HTML-->
<template>
  <div class="adds">
    <button @click="showCatForm" class="add-category">New Category</button>
    <button @click="showPurForm" class="add-purchase">New Purchase</button>
  </div>

  

  <div class="form-container">
    <form v-if="isCatForm" class="add-form" method="post">
      <dd>Name: <input id="categoryName" type="text" name="name"></dd>
      <dd>Budget: <input id="categoryBudget" type="number" name="budget"></dd>
      <dd><input @click="sendNewCat" id="sendButton" type="button" value="Add"></dd>
    </form>

    <form v-if="isPurForm" class="add-form" method="post">
      <dd>Description: <input id="purchaseDesc" type="text" name="desc"></dd>
      <dd>Amount: <input id="purchaseAmount" type="number" name="amount"></dd>
      <dd>Date: <input id="purchaseDate" type="date" name="date"></dd>
      <dd>Category:    
        <select name="category" id="purCatName">
          <option value="">--Choose your category--</option>
          <option v-for="cat in cats" :key="cat.id" v-bind:value="cat.name"> {{ cat.name }}</option>
        </select>
      </dd>
      <dd><input @click="sendNewPur" id="sendButton" type="button" value="Add"></dd>
    </form>
  </div>

  <h1>Categories:</h1>
  <div class="category-list">
    <ul>
      <li v-for="cat in cats" :key="cat.id">
        {{ cat.name }}: 
        <span v-if="cat.budget">${{ cat.budget }}</span>
        <span v-else>--</span>  
      </li>
    </ul>
  </div>

  <h1>Purchases:</h1>
  <div class="purchase-list">
    <ul>
      <li v-for="pur in purs" :key="pur.id" :class="{ 'over-budget': pur.overBudget }">
        [{{ pur.date }}] ${{ pur.amount }} {{ pur.desc }} ({{ pur.category }})
      </li>
    </ul>
  </div>
</template>

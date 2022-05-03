<template>
  <div class="flex items-center min-h-screen bg-white dark:bg-gray-900">
    <div class="container mx-auto">
      <div class="max-w-md mx-auto my-10">
        <div class="text-center">
          <h1
            class="my-3 text-3xl font-semibold text-gray-700 dark:text-gray-200"
          >
            登录
          </h1>
          <p class="text-gray-500 dark:text-gray-400">鲁班：原型托管平台</p>
        </div>
        <div class="m-7">
          <form action="">
            <div class="mb-6">
              <label
                for="username"
                class="block mb-2 text-sm text-gray-600 dark:text-gray-400"
                >用户名</label
              >
              <input
                type="username"
                name="username"
                id="username"
                placeholder=""
                class="w-full px-3 py-2 placeholder-gray-300 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-100 focus:border-indigo-300 dark:bg-gray-700 dark:text-white dark:placeholder-gray-500 dark:border-gray-600 dark:focus:ring-gray-900 dark:focus:border-gray-500"
                v-model="username"
              />
            </div>
            <div class="mb-6">
              <div class="flex justify-between mb-2">
                <label
                  for="password"
                  class="text-sm text-gray-600 dark:text-gray-400"
                  >密码</label
                >
              </div>
              <input
                type="password"
                name="password"
                id="password"
                placeholder=""
                class="w-full px-3 py-2 placeholder-gray-300 border border-gray-300 rounded-md focus:outline-none focus:ring focus:ring-indigo-100 focus:border-indigo-300 dark:bg-gray-700 dark:text-white dark:placeholder-gray-500 dark:border-gray-600 dark:focus:ring-gray-900 dark:focus:border-gray-500"
                v-model="password"
              />
            </div>
            <div class="mb-6">
              <button
                type="button"
                class="w-full px-3 py-4 text-white bg-indigo-500 rounded-md focus:bg-indigo-600 focus:outline-none"
                @click="login"
              >
                登录
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
  <!-- Footer Mentions -->
  <div class="fixed bottom-5 w-full text-center text-gray-400">
    Crafted with ♡ by
    <a class="text-blue-500" target="_blank" href="https://github.com/stanzhai"
      >StanZhai</a
    >
  </div>
</template>

<script>
import { post } from '@/utils/http'
import { ElMessage } from 'element-plus'

export default {
  data() {
    return {
      username: "",
      password: "",
    };
  },
  methods: {
    login() {
      post("/api/auth/login", this).then((res) => {
        if (res.success) {
          let userInfo = res.payload
          console.log(userInfo)
          this.$router.push("/")
        } else {
          ElMessage.error(res.message)
        }
      });
    },
  },
};
</script>

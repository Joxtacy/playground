<script lang="ts">
    const dogImg = require("./dog");
    const styling = `
    margin-top: 8px;
    margin-horizontal: 16px;
    padding-horizontal: 8px;
    `;
</script>

<view style={styling}>
    <text wordWrap={true}>
        Edit App.svelte to make changes to this screen. Then come back to see your changes. The app will restart with each change, thanks to Live Reload 🔥
    </text>
    <text>
      {`
        <p style="color: rgb(255,72,38);">
          <center>
            <img src="${dogImg}" alt="doggy" />
          </center>
          <center>You can even use <i><strong>rich HTML text</strong></i> like this if you want 😎</center>
        </p>
        <hr />
      `}
    </text>
</view>

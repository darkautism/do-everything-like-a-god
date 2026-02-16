#!/bin/bash
URL="https://darkautism.github.io/do-everything-like-a-god/"
BROWSER="/home/kautism/.npm-global/bin/agent-browser"

echo "Step 1: Testing Direct Path (SEO SSG)..."
$BROWSER open "${URL}base64/"
$BROWSER wait 3000
TITLE=$($BROWSER eval "document.title")
echo "Page Title: $TITLE"
CONTENT=$($BROWSER eval "document.querySelector('main').innerText")
if [[ -z "$CONTENT" ]]; then
    echo "FAIL: Content is empty on direct access"
    $BROWSER close
    exit 1
else
    echo "PASS: Direct path loaded content"
fi

echo "Step 2: Testing Sidebar Link..."
$BROWSER click "a:has-text('JSON Tool')"
$BROWSER wait 2000
NEW_URL=$($BROWSER get url)
echo "URL after click: $NEW_URL"
if [[ "$NEW_URL" == *"/do-everything-like-a-god/json"* ]]; then
    echo "PASS: Sidebar link kept base path"
else
    echo "FAIL: Sidebar link jumped out of base path"
    $BROWSER close
    exit 1
fi

echo "Step 3: Testing WASM Logic (JSON Prettify)..."
$BROWSER fill "textarea" '{"god":"mode"}'
$BROWSER click "button:has-text('Prettify')"
$BROWSER wait 500
RESULT=$($BROWSER eval "document.querySelectorAll('textarea')[1].value")
if [[ "$RESULT" == *"\"god\": \"mode\""* ]]; then
    echo "PASS: WASM Logic verified (JSON Prettified)"
else
    echo "FAIL: WASM Logic error (Got: $RESULT)"
    $BROWSER close
    exit 1
fi

$BROWSER close
echo "ALL TESTS PASSED."

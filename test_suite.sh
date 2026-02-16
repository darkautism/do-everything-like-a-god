#!/bin/bash
URL="https://darkautism.github.io/do-everything-like-a-god/"
BROWSER="/home/kautism/.npm-global/bin/agent-browser"

echo "Checking site availability..."
STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$URL")
if [ "$STATUS" != "200" ]; then
    echo "Site returned $STATUS. Deployment might still be in progress."
fi

echo "Test 1: Direct Subdirectory Access (SEO Verification)"
$BROWSER open "${URL}json/"
$BROWSER wait 3000
TITLE=$($BROWSER eval "document.title")
echo "Captured Title: $TITLE"
if [[ "$TITLE" == *"JSON"* ]]; then
    echo "PASS: SEO Title Injected correctly."
else
    echo "FAIL: SEO Title missing or incorrect."
fi

echo "Test 2: Relative Link Navigation"
$BROWSER click "a:has-text('Base64')"
$BROWSER wait 2000
CURRENT_URL=$($BROWSER get url)
echo "Current URL: $CURRENT_URL"
if [[ "$CURRENT_URL" == *"/base64"* ]]; then
    echo "PASS: Relative link navigated correctly within base."
else
    echo "FAIL: Link jump out of base path."
fi

echo "Test 3: WASM Logic verification"
$BROWSER fill "textarea" "GOD MODE"
$BROWSER click "button:has-text('Encode')"
$BROWSER wait 500
RESULT=$($BROWSER eval "document.querySelectorAll('textarea')[1].value")
if [ "$RESULT" == "R09EIE1PREU=" ]; then
    echo "PASS: WASM Logic (Base64 Encode) working."
else
    echo "FAIL: WASM Logic error (Got: $RESULT)"
fi

$BROWSER close
echo "All Done."

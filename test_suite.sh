#!/bin/bash
URL="https://darkautism.github.io/do-everything-like-a-god/"
BROWSER="/home/kautism/.npm-global/bin/agent-browser"

echo "Waiting for $URL to be live..."
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$URL")
    if [ "$STATUS" == "200" ]; then
        echo "Site is live!"
        break
    fi
    echo "Attempt $((RETRY_COUNT+1))/$MAX_RETRIES: Site returned $STATUS. Waiting 20s..."
    sleep 20
    RETRY_COUNT=$((RETRY_COUNT+1))
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo "Timeout waiting for deployment."
    exit 1
fi

echo "Running Browser Tests..."
$BROWSER open "$URL"
$BROWSER wait 2000
$BROWSER snapshot -i | grep -q "做甚麼都有如神助" && echo "PASS: Homepage Loaded" || echo "FAIL: Homepage Loaded"

echo "Testing Base64 Link..."
$BROWSER click @e3
$BROWSER wait 1000
CURRENT_URL=$($BROWSER get url)
if [[ "$CURRENT_URL" == *"base64"* ]]; then
    echo "PASS: Base64 Route Accessible ($CURRENT_URL)"
else
    echo "FAIL: Base64 Route Invalid ($CURRENT_URL)"
    exit 1
fi

echo "Testing Base64 Logic..."
$BROWSER fill @e1 "GOD MODE"
$BROWSER click @button --name "Encode"
$BROWSER wait 500
RESULT=$($BROWSER get value @e2)
if [ "$RESULT" == "R09EIE1PREU=" ]; then
    echo "PASS: Base64 Logic Working"
else
    echo "FAIL: Base64 Logic Error (Got: $RESULT)"
fi

echo "Testing Direct Access SEO Path..."
$BROWSER open "${URL}json/"
$BROWSER wait 2000
$BROWSER snapshot -i | grep -q "JSON Tool" && echo "PASS: SEO Path Working" || echo "FAIL: SEO Path Broken"

$BROWSER close
echo "All Tests Completed."

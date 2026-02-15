#!/bin/bash
export PATH=$PATH:/home/kautism/.npm-global/bin
URL="http://localhost:8080" # Default Trunk port

echo "--- TDD Test Suite: DO EVERYTHING LIKE A GOD ---"

# Test 1: Homepage & i18n
echo "[Test 1] Checking Homepage Content..."
agent-browser open $URL
agent-browser wait --load networkidle
agent-browser snapshot -i | grep -q "做甚麼都有如神助" && echo "PASS: Homepage Loaded" || echo "FAIL: Homepage Loaded"

# Test 2: Base64 Encoding
echo "[Test 2] Checking Base64 Tool..."
agent-browser open "$URL/base64"
agent-browser wait --load networkidle
agent-browser snapshot -i | grep -q "Base64 有如神助" && echo "PASS: Base64 Loaded" || echo "FAIL: Base64 Loaded"
agent-browser fill "textarea[placeholder='...']" "Hello"
agent-browser wait 500
agent-browser snapshot -i | grep -q "SGVsbG8=" && echo "PASS: Base64 Encoding" || echo "FAIL: Base64 Encoding"

# Test 3: HTML Escape
echo "[Test 3] Checking HTML Escape Tool..."
agent-browser open "$URL/html-escape"
agent-browser wait --load networkidle
agent-browser snapshot -i | grep -q "HTML Escape 有如神助" && echo "PASS: HTML Escape Loaded" || echo "FAIL: HTML Escape Loaded"
agent-browser fill "textarea[placeholder='<div>...</div>']" "<div>"
agent-browser wait 500
agent-browser snapshot -i | grep -q "&lt;div&gt;" && echo "PASS: HTML Escape" || echo "FAIL: HTML Escape"

agent-browser close
echo "--- Test Suite Finished ---"

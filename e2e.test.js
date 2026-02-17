const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');

const PROJECT_DIR = path.join(__dirname);
const BASE_URL = 'http://localhost:3456/do-everything-like-a-god/';

async function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function build() {
    return new Promise((resolve, reject) => {
        const proc = spawn('trunk', ['build'], { cwd: PROJECT_DIR, stdio: 'pipe' });
        let output = '';
        proc.stdout.on('data', d => output += d);
        proc.stderr.on('data', d => output += d);
        proc.on('close', code => {
            if (code === 0) {
                const fs = require('fs');
                const publicDir = path.join(PROJECT_DIR, 'public');
                const distDir = path.join(PROJECT_DIR, 'dist');
                if (fs.existsSync(publicDir)) {
                    fs.readdirSync(publicDir).forEach(file => {
                        fs.copyFileSync(path.join(publicDir, file), path.join(distDir, file));
                    });
                }
                resolve(output);
            } else {
                reject(new Error(`Build failed`));
            }
        });
    });
}

async function startServer() {
    return new Promise((resolve, reject) => {
        const proc = spawn('npx', ['-y', 'serve', '-s', '-l', '3456'], {
            cwd: path.join(PROJECT_DIR, 'dist'),
            stdio: ['pipe', 'pipe', 'pipe']
        });
        let output = '';
        proc.stdout.on('data', d => { output += d.toString(); });
        proc.stderr.on('data', d => { output += d.toString(); });
        
        const check = setInterval(() => {
            if (output.includes('Accepting')) {
                clearInterval(check);
                resolve(proc);
            }
        }, 200);
        
        setTimeout(() => {
            clearInterval(check);
            if (!proc.killed) resolve(proc);
        }, 10000);
    });
}

async function runTests() {
    let server = null;
    let browser = null;
    
    try {
        console.log('Building project...');
        await build();
        
        console.log('Starting server...');
        server = await startServer();
        await sleep(2000);
        
        console.log('Launching browser...');
        browser = await chromium.launch({ headless: true });
        const page = await browser.newPage();
        await page.setViewportSize({ width: 1280, height: 800 });
        
        page.on('console', msg => console.log('Browser console:', msg.text()));
        page.on('pageerror', err => console.log('Browser error:', err));
        
        let passed = 0;
        let failed = 0;
        
        // Test 1: Home page
        console.log('\n[Test 1] Home page loads...');
        await page.goto(BASE_URL, { timeout: 15000, waitUntil: 'networkidle' });
        await sleep(1000);
        
        if ((await page.content()).includes('UTILITIES') || (await page.content()).includes('GOD')) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 2: Navigation
        console.log('\n[Test 2] Navigation links...');
        const linkCount = await page.locator('a').count();
        if (linkCount >= 16) {
            console.log(`  ✓ PASS: ${linkCount} links`);
            passed++;
        } else {
            console.log(`  ✗ FAIL: Only ${linkCount} links`);
            failed++;
        }
        
        // Test 3: Base64 encoding
        console.log('\n[Test 3] Base64 encoding...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Base64');
        await page.waitForTimeout(3000);
        
        const base64Tas = await page.locator('textarea').all();
        if (base64Tas.length >= 2) {
            await base64Tas[0].fill('Hello');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            const base64Out = await base64Tas[1].inputValue();
            if (base64Out.includes('SGVsbG8=')) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log(`  ✗ FAIL: "${base64Out}"`);
                failed++;
            }
        } else {
            console.log('  ✗ FAIL: No textareas');
            failed++;
        }
        
        // Test 4: Base32
        console.log('\n[Test 4] Base32 encoding...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Base32');
        await page.waitForTimeout(3000);
        
        const base32Tas = await page.locator('textarea').all();
        if (base32Tas.length >= 2) {
            await base32Tas[0].fill('Hi');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            const out = await base32Tas[1].inputValue();
            if (out.length > 0) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 5: Base58
        console.log('\n[Test 5] Base58 encoding...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Base58');
        await page.waitForTimeout(3000);
        
        const base58Tas = await page.locator('textarea').all();
        if (base58Tas.length >= 2) {
            await base58Tas[0].fill('Test');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            if ((await base58Tas[1].inputValue()).length > 0) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 6: HTML Escape
        console.log('\n[Test 6] HTML Escape...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=HTML Escape');
        await page.waitForTimeout(3000);
        
        const htmlTas = await page.locator('textarea').all();
        if (htmlTas.length >= 2) {
            await htmlTas[0].fill('<div>');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            const out = await htmlTas[1].inputValue();
            if (out.includes('&lt;')) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 7: URL Encode
        console.log('\n[Test 7] URL Encode...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=URL Escape');
        await page.waitForTimeout(3000);
        
        const urlTas = await page.locator('textarea').all();
        if (urlTas.length >= 2) {
            await urlTas[0].fill('hello world');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            if ((await urlTas[1].inputValue()).includes('%20')) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 8: Hash
        console.log('\n[Test 8] Hash generation...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Hash');
        await page.waitForTimeout(3000);
        
        await page.locator('textarea').first().fill('test');
        await sleep(500);
        
        if (await page.locator('.hash-results input').count() >= 5) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 9: JSON
        console.log('\n[Test 9] JSON prettify...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=JSON');
        await page.waitForTimeout(3000);
        
        const jsonTas = await page.locator('textarea').all();
        if (jsonTas.length >= 2) {
            await jsonTas[0].fill('{"a":1,"b":2}');
            await page.locator('.box').first().locator('button.btn').first().click();
            await sleep(500);
            
            const out = await jsonTas[1].inputValue();
            if (out.includes('\n')) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 10: UUID
        console.log('\n[Test 10] UUID generation...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=UUID');
        await page.waitForTimeout(3000);
        
        const initial = await page.locator('.uuid-display').textContent();
        await page.locator('button.btn').click();
        await sleep(500);
        
        const after = await page.locator('.uuid-display').textContent();
        if (after !== initial && after.includes('-')) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 11: Regex
        console.log('\n[Test 11] Regex tester...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Regex');
        await page.waitForTimeout(3000);
        
        await page.locator('input.regex-input').fill('\\d+');
        await page.locator('textarea').fill('abc123def');
        await sleep(500);
        
        const regexResult = await page.locator('.regex-result').textContent();
        if (regexResult.includes('123')) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 12: Timestamp
        console.log('\n[Test 12] Timestamp conversion...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Timestamp');
        await page.waitForTimeout(3000);
        
        await page.locator('input').first().fill('1704067200');
        await page.locator('button.btn').first().click();
        await sleep(500);
        
        if ((await page.locator('.iso-display').textContent()).includes('2024')) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 13: Base Converter
        console.log('\n[Test 13] Base converter...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Base Conv');
        await page.waitForTimeout(3000);
        
        if (await page.locator('.base-result').count() > 0) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 14: Diff
        console.log('\n[Test 14] Diff tool...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Diff');
        await page.waitForTimeout(3000);
        
        if (await page.locator('.box').count() >= 2) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 15: Cron
        console.log('\n[Test 15] Cron parser...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Cron');
        await page.waitForTimeout(3000);
        
        await page.locator('input').first().fill('* * * * *');
        await page.locator('button.btn').first().click();
        await sleep(500);
        
        if (await page.locator('.cron-desc').count() > 0) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 16: AES
        console.log('\n[Test 16] AES encryption UI...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=AES');
        await page.waitForTimeout(3000);
        
        if ((await page.locator('textarea').count()) >= 2) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 17: JWT
        console.log('\n[Test 17] JWT decode...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=JWT');
        await page.waitForTimeout(3000);
        
        const jwtTas = await page.locator('textarea').all();
        if (jwtTas.length >= 1) {
            const testToken = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
            await jwtTas[0].fill(testToken);
            await sleep(500);
            
            if ((await page.locator('.box').count()) >= 3) {
                console.log('  ✓ PASS');
                passed++;
            } else {
                console.log('  ✗ FAIL');
                failed++;
            }
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Test 18: Image Base64
        console.log('\n[Test 18] Image Base64 UI...');
        await page.goto(BASE_URL, { timeout: 20000, waitUntil: 'networkidle' });
        await page.waitForTimeout(2000);
        await page.click('text=Image Base64');
        await page.waitForTimeout(3000);
        
        if ((await page.locator('input[type="file"]').count()) >= 1) {
            console.log('  ✓ PASS');
            passed++;
        } else {
            console.log('  ✗ FAIL');
            failed++;
        }
        
        // Summary
        console.log('\n========================================');
        console.log(`Total: ${passed + failed}, Passed: ${passed}, Failed: ${failed}`);
        console.log('========================================\n');
        
        // SEO Tests
        console.log('\n[SEO Tests]');
        
        await page.goto(BASE_URL, { timeout: 10000, waitUntil: 'networkidle' });
        await page.waitForTimeout(1000);
        
        const title = await page.title();
        console.log(`  Title: ${title}`);
        
        const metaDesc = await page.locator('meta[name="description"]').first().getAttribute('content');
        console.log(`  Meta description: ${metaDesc ? '✓ Present: ' + metaDesc.substring(0, 50) + '...' : '✗ Missing'}`);
        
        const ogTitle = await page.locator('meta[property="og:title"]').getAttribute('content');
        console.log(`  Open Graph title: ${ogTitle ? '✓ Present' : '✗ Missing'}`);
        
        const ogDesc = await page.locator('meta[property="og:description"]').getAttribute('content');
        console.log(`  Open Graph description: ${ogDesc ? '✓ Present' : '✗ Missing'}`);
        
        const twitterCard = await page.locator('meta[property="twitter:card"]').count() > 0
            ? await page.locator('meta[property="twitter:card"]').getAttribute('content') : null;
        console.log(`  Twitter Card: ${twitterCard ? '✓ Present: ' + twitterCard : '✗ Missing'}`);
        
        const canonical = await page.locator('link[rel="canonical"]').count() > 0
            ? await page.locator('link[rel="canonical"]').getAttribute('href') : null;
        console.log(`  Canonical URL: ${canonical ? '✓ Present' : '✗ Missing'}`);
        
        const favicon = await page.locator('link[rel="icon"]').count() > 0
            ? await page.locator('link[rel="icon"]').getAttribute('href') : null;
        console.log(`  Favicon: ${favicon ? '✓ Present' : '✗ Missing'}`);
        
        // PWA Tests
        console.log('\n[PWA Tests]');
        
        const manifest = await page.locator('link[rel="manifest"]').count() > 0
            ? await page.locator('link[rel="manifest"]').getAttribute('href') : null;
        console.log(`  Web App Manifest: ${manifest ? '✓ Present: ' + manifest : '✗ Missing'}`);
        
        const themeColor = await page.locator('meta[name="theme-color"]').count() > 0
            ? await page.locator('meta[name="theme-color"]').getAttribute('content') : null;
        console.log(`  Theme Color: ${themeColor ? '✓ Present: ' + themeColor : '✗ Missing'}`);
        
        const appleCapable = await page.locator('meta[name="apple-mobile-web-app-capable"]').count() > 0;
        console.log(`  Apple Web App Capable: ${appleCapable ? '✓ Present' : '✗ Missing'}`);
        
        // hreflang Tests
        console.log('\n[hreflang Tests]');
        
        const hreflangCount = await page.locator('link[rel="alternate"][hreflang]').count();
        console.log(`  hreflang tags: ${hreflangCount > 0 ? '✓ Present (' + hreflangCount + ')' : '✗ Missing'}`);
        console.log('\n');
        
        process.exit(failed > 0 ? 1 : 0);
        
    } catch (e) {
        console.error('Error:', e);
        process.exit(1);
    } finally {
        if (browser) await browser.close();
        if (server) server.kill();
    }
}

runTests();

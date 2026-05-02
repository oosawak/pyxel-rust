# GitHub Pages Configuration

## Steps to Enable GitHub Pages

1. **Go to Repository Settings**
   - Navigate to https://github.com/oosawak/pyxel-rust/settings/pages

2. **Configure Pages Source**
   - Source: `Deploy from a branch`
   - Branch: `master` (or `main`)
   - Folder: `/docs`
   - Click "Save"

3. **Wait for Deployment**
   - GitHub will build and deploy your site
   - It should be available at: `https://oosawak.github.io/pyxel-rust/`

4. **Access Your Game**
   - Cubeboy: `https://oosawak.github.io/pyxel-rust/cubeboy/`

## Automatic Deployment

Every push to the master branch will automatically rebuild and deploy the site.

## Local Testing

To test the site locally:

```bash
# Using Python 3
cd docs
python3 -m http.server 8000

# Then visit http://localhost:8000/cubeboy/
```

## Future Enhancements

Once WASM build is complete:
1. Run `pyxel-rust app2html cubeboy`
2. Copy `web/pkg/*` to `docs/cubeboy/pkg/`
3. Commit and push
4. Game will be live at GitHub Pages URL

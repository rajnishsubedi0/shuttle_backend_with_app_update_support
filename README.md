# Bhajan sangraha backend with auto update feathre
This is working backend of bhajan sangra with auto update feature

# First of all clone this repo and navigate its folder. If there is ```.shuttle``` folder into the project then delete that folder
 * Then authorize the terminal with following command
   ```shuttle login```
 * After that enter following code
     ```shuttle deploy```
 * Then it will ask to replace existing project or create new project, and navigate to the `CREATE NEW PROJECT` and select that and type project name.
   It will create hidden folder ```.shuttle``` into the main project folder.
 * Then project should automatically deployed. If not then enter ```shuttle deploy```
 * After that to upload the app, enter following command in the terminal
   ```
   curl -X POST -F "file=@/home/rkant/Downloads/app-release.apk" https://check-version-number-axvy.shuttle.app/route
   ```
 * After that enter following url to Download app
   ```
   https://check-version-number-axvy.shuttle.app/route/1
   ```
* And POST following JSON so app can fetch and compare app version on following link.
  ```
  https://check-version-number-axvy.shuttle.app/route/version
  ```
  JSON Link
  ```
  { "version_number_id":4 }
  ```
* To fetch above json, use following url
  ```
  https://check-version-number-axvy.shuttle.app/route/version/1
  ```

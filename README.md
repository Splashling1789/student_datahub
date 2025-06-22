## License
This project is licensed under the _GNU AFFERO GENERAL PUBLIC LICENSE_, version 3.0. Check [LICENSE](./LICENSE) for more details.
# Student datahub
This command-line tool allows you to keep track of the time spent on all the subjects of your academic year. It also has export methods, in order to do some data analysis of your time.
It is a personal project that aims to keep motivation high in college periods. It may be adapted for a less specific approach in the future.

## Installation
You can install it through cargo, either from crates.io index:
```bash
cargo install student_datahub
```
Or from source code, opening a terminal in the project folder:
```bash
cargo install --path .
```

## Usage
To start everything, you can create a study plan, which is basically a time period in which you will be studying.
```bash
student_datahub plan start [start date] (end date) (plan description)
```
If the local date is in the date range of the plan, you can start adding subjects:
```bash
student_datahub subject add (short name) (complete name)
```
The subject's short name is used as an identifier, and it is unique for each study plan. The complete name can have spaces without being quoted.

Once you have all of your subjects added, you can now add some time. Maybe you've dedicated one hour of studying complex analysis? Then:
```bash
student_datahub add CompAn 60
```
Where CompAn is the short name of the subject you created. The syntax is the following:
```bash
student_datahub add/substract/set [date] (Short name/Subject ID) (amount)
```
Over time, you may want to see how you're doing. When you want that, just run:
```bash
student_datahub status
```
And a summary of your study sessions will be displayed. Here is an example:
```
Current plan: College-2S2 (ID:1)
28-03-2025 - ... - 22-05-2025 - ... - 31-05-2025
------------------------------------------------------------
	You have studied a total amount of 6h 0min:
	 * 6h 0min were dedicated to Ordinary differential equations
------------------------------------------------------------
	This week you have studied 20h 40min:
	 * 6h 20min were dedicated to Databases
	 * 12h 45min were dedicated to Ordinary differential equations
	 * 1h 35min were dedicated to Parallel programming

	 - You've studied 23.4% more than last week.
	 - 119.5% more than weekly average.
```

And now for the best part: You can export your data as csv files for later data analysis. Just run:
```bash
student_datahub export (all/monthly/weekly/daily)
```
That should create a folder `$HOME$/.student_datahub/{date}_{time}_{plan description}` where all your csv files will be, with this format:
```csv
date,Algth,Dbs,DfE,Prb,SfI,PlP,Net <- Each column is the short name of a subject
22-04-2025,0,60,0,30,45,30,30
23-04-2025,10,0,0,0,20,30,30
...
```
This is the 'daily' format. In 'weekly' and 'monthly' one line is one week or month.

## Date format
The current date format is '%d-%m-%Y'. If you would rather use another such as '%m-%d-%Y' you can change constant `FORMAT` in main.rs before compiling.

## Contributing
While this is a personal project mainly done for learning, I would appreciate any suggestions or issue reports. Feel free to tell me I'm wrong and why, always in a good manner.
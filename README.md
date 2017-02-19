# MacroBM - A tool to run MacroBenchmarks

This repository hosts a tool called `macrobm` that will run benchmarks for
arbitray commands and is able to compare different versions of a program
performancewise.

[!Usage Example](example.png)

## Quickstart

It is inspired by the `CMake` tools and `linux perf` and tries to be useable
with almost no configuration!

All it needs is a `benchmarks.yml` file, where the commands for the benchmark
are configured.

```yaml
cases:
    - name: "sleep_long"                  # optional, will default to command
      command: "/bin/sleep"               # command to execute, NO SHELL SCRIPT
      args: ["5",]                        # list of arguments passed to the command
      count: 5                            # how often the command is run
```

```sh
$ macrobm # directory must contain benchmarks.yml
> Running macro benchmarks 1 threads
> Scheduling sleep_long for 5 runs
> Finished running benchmarks!
> Runs     Min        Avg       Dev      Max             Name        
>   5       5.01       5.01    +-0.0 %    5.01         sleep_long
$ macrobm report # inspects results.yml, same as report output from bmrun
$ macrobm diff ground_truth.yml # compare ground_truth.yml with results.yml
```

All input and output parameters, especially filenames, can be specified with
command line arguments, see `macrobm --help` for more. Especially you can
utilize many cores of your machine, using the `-jN` parameter. Note that this
might have impact on your execution time of the programs, depending on the
thread count you configured and your hardware!

Currently the errorhandling is shaky, whenevery panics occur, e.g. the
execution directory does not exist, the program is bad at recovering. Hit
CTRL-C and fix your configuration! This will improve once it is ready to get
released.

## Usage

Values defined outside the `cases` subsection will be used as default
arguments. So you can configure everything outside cases, and just vary with
your `args` when comparing one program with different configurations.

```yaml
count: 30                                          
command: "../ulf.x"                                
cases:                                             
    - name: "hReactor_ct"                          
      args: ["-f", "hReactor/hReactor_ct.ulf"]     
                                                   
    - name: "hReactor_ct_chem"                     
      args: ["-f", "hReactor/hReactor_ct_chem.ulf"]
                                                   
    - name: "hReactor_eg"                          
      args: ["-f", "hReactor/hReactor_eg.ulf"]     
                                                   
    - name: "hReactor_uc"                          
      args: ["-f", "hReactor/hReactor_uc.ulf"]     
```

This is a configuration file for a numeric code. This evaluates the runtime of a 
flame-solver depending on the solverbackend.
Again, everything is configurable "above" cases, defaulting it for all cases
and can be overwritten by each case.

Using the `macrobm diff` command one can see that AMD processors dont have the
real core count they pretend to have :)

```sh
# running on an AMD FX-6300 [I am a poor student :(]
$ macrobm -o results_j2.yml -j2
$ macrobm -o results_j6.yml -j6
$ macrobm diff results_j2.yml results_j6.yml
>                results_j2.yml                 =====================                 results_j6.yml                 
> Runs     Min        Max       Dev      Avg             Name            Avg       Dev      Min        Max      Runs 
>  30      0.75       0.81    +-1.1 %    0.77        hReactor_ct         1.03    +-3.6 %    0.97       1.16      30  
>  30      0.63       0.69    +-1.7 %    0.65      hReactor_ct_chem      0.88    +-3.4 %    0.84       0.98      30  
>  30      4.12       5.27    +-3.7 %    4.30        hReactor_eg         6.07    +-2.5 %    5.76       6.56      30  
>  30      3.83       4.03    +-0.9 %    3.90        hReactor_uc         5.69    +-3.2 %    4.89       6.27      30  
```

You can clearly see that running with only two threads results into lower
runtimes. So the threads had to block in the `-j6` case. On this processor 2 cores 
share one FPU, explaining this effect.

## Todo

- command to create a configuration as starting point as example to modify
- allow groups in the config, hierarchy, until "cases" is found. These are then
  used to configure the runs, the rest are namespaces
- environment variables must be handled correct
- check subcommand, that will compare the statistics against a defined
  requirement and return 0 or -1 if the requirements are met or not. usefull
  for ci

## Way later
- submit subcommand to build a database of historic data

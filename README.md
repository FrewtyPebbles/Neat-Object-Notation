# Neat Object Notation 0.3.1

 A smart, modular and readable configuration file format for complex multifile solutions.

## Comments

```
| Any lines which are to be commented out must start with a pipe character.
| The pipe must ALWAYS be at the beginning of the line for comments.
```

## Global Scope

 By default the global scope of a Neat config file is a dictionary, if you wish to specify otherwise you must put this somewhere in your .neat file on its own line

```
~list
```

## Labeled Structures

```
[This is where you write the key associated with your dictionary]

	"This is a key to an inline dictionary":{"This is the key to an inline list":()}

| This [-] token denotes the end of a dictionary.
[-]

<This is where you write the key associated with your list>

|	The line below is the 0th index of this labeled list and is a list with a single item,
|	That single item is an empty dictionary
	({})

| This <-> token denotes the end of a list.
<->
```

## Unlabeled Structures

If you wish to create an unlabeled structure vertically you can do so like this:

```
~list

{
	"Some key":29873198273
}

```

Another example:

```
<section name>
	{
		[inner section name]
			"some key": True
		[-]
		"another key": "abc"
	}
<->
```

## Modules

Importing only specific sections of a module:

```
| This file is called filename.neat
<section name>
	{
		[inner section name]
			"some key": True
		[-]
		"another key": "abc"
	}
<->
```

```
|this is where we are importing the module
mod filename : 'section name'.0.'inner section name'

| Alternate syntax

* foldername.filename : 'section name'.0.'inner section name'
```

Importing a whole module:

```
|this file is called module.neat
[section]
	1:"abc"
[-]
```

```
| This is where we import module.neat
mod module

[another section]
	"def":2
[-]

| Result:
| {"module":{"section":{"1":"abc"}},"another section":{"def":2}}
```

## Alias

Aliases can be used to add items to sections outside of that section and its parent.
The left hand side of the : is the alias name.  The right hand side of the : is the alias path.

```
<section name>
	{
		[inner section name]
			"some key": True
		[-]
		"another key": "abc"
	}
<->

| this is the alias declaration
alias alias_name : [section name] 0 [inner section name]

| the name of the alias, in this case alias_name, marks the start of an alias section.
alias_name
	"some other key": false
| The /-/ token marks the end of an alias section
/-/

| Result:
| {"section name":[{"inner section name":{"some key":True,"some other key":False},"another key":"abc"}]}
```

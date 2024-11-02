1. CSE 542 Fall 2024 Lab 2

2. Sahil Athalye: a.sahil@wustl.edu
Varad Deouskar: varad@wustl.edu

3. MODULES: 
    - Nearly all constant variable initializations in declarations.rs were moved to main.rs as most of them were exclusively used in main.rs. The Play type declaration, SCRIPT_GEN_FAILURE error code, and SHOULD_COMPLAIN atomic boolean were all declared public as they were used in multiple files.

    - To ensure compilation, a variety of use statements with the crate paths to the desired variables were included at the top of script_gen.rs and main.rs. The script gen function was also required to be included in main.rs from script_gen.rs. 

    - There were three points where the script could whinge. In add_config() if there were and incorrect number of config tokens in a line and in add_script_line() if the token could not be parsed into a number. Instead of printing to the standard output stream, whinges will now print to the standard error stream. 

    - A key design decision was to not combine the helper functions and script_gen function into main.rs even though script_gen is only used called in main.rs. This decision is being made to compartmentalize the code better and allow an easier understanding of the two distinct processing steps that take place to make this program possible. 

4. Structs: 

    - In our refactoring, we structured the code around two main structs, Play and Player, to organize the play's data and behavior more effectively. The Player struct was designed to hold each character's name, lines (stored in PlayLines), and the index of the current line. Within the Player struct, we implemented several methods. The add_script_line method, a private function, adds a parsed line to the player’s PlayLines. The public prepare method reads and processes each character’s lines by calling add_script_line and then sorting the lines to account for any out-of-order entries. The speak method prints the player’s next line, checks for character name changes, and updates the current line index, while the next_line method returns the number of the upcoming line or None if there are no more lines.

    -  The Play struct itself holds the scene title and a vector of Player instances and includes methods that operate on these. We designed process_config to iterate through the play configuration, creating and preparing a new Player instance for each character, and pushing each player into the characters vector. This method returns an error if any prepare call fails, ensuring robust error handling. The recite method in Play delivers each line in order by checking the next line number across all players. In addition, it manages missing and duplicate lines as specified, logging any issues to the error stream if whinge mode is enabled.

    - During the refactoring process, we encountered several design challenges. A primary challenge was handling mutable borrowing correctly when iterating through players in recite. To resolve this, we ensured that next_player was a mutable reference and used &mut self.characters in the loop to enable mutable access. Another challenge was handling missing and duplicate lines, which we addressed by using Option types and conditional checks within recite to control the flow of line delivery. Any missing line numbers are incremented accordingly, and any discrepancies are logged when whinge mode is active.
    
    - This struct-based approach not only helped us manage ownership and borrowing effectively, as required by Rust, but also created a more modular and cohesive design, where each struct is responsible for its own data and associated behavior. This led to a clear, maintainable structure that supports ordered recitation of lines in the play.


5. Return Wrapper
    - Wraps a u8 exit code

    - Implements Termination trait to control program exit behavior

    - Prints error code to stderr when non-zero

    - Converts exit code to ExitCode for system return

ReturnWrapper encapsulates error handling logic and provides consistent error reporting across the program.

SceneFragment struct manages individual scenes with:
    - Title and character list 
    - Configuration processing
    - Line-by-line recitation with proper ordering
    - Smart character transitions:
    - Announces only entering/exiting characters between scenes
    - Handles first/last scenes specially
    - Exits characters in reverse order

Main challenges addressed:
    - Line number sequencing across scenes
    - Character continuity between scenes
    - Error handling in configuration and preparation

Testing Approach
The testing methodology was basically the same as lab 1.

We also used the "2>" to make sure out errors are going to stderr.

Variety of Inputs: We created multiple configuration and character files to cover edge cases, including well-formed and malformed content.

For error checking we had test cases like invalid number of program args, invalid lines such as having invalid line index, invalid file names and extra whitespaces.

Automation: Used shell scripts to automate testing with different inputs and capture outputs for comparison.

Validation: Manually verified outputs against expected results, ensuring correct ordering and formatting.

Observations from Testing

The program effectively handles various error conditions and warns of error when whinge is enabled.


Output - 
[varad@archlinux debug]$ ./lab2 partial_hamlet_act_ii_script.txt whinge
Hamlet Prince of Denmark ACT II Scene I A room in Polonius house by William Shakespeare
[Enter Polonius.]
[Enter Reynaldo.]
Warning: Missing line number 0

Polonius
Give him this money and these notes, Reynaldo.

Reynaldo
I will, my lord.

Polonius
You shall do marvellous wisely, good Reynaldo,
Before You visit him, to make inquiry
Of his behaviour.

Reynaldo
My lord, I did intend it.

Polonius
Marry, well said; very well said. Look you, sir,
Enquire me first what Danskers are in Paris;
And how, and who, what means, and where they keep,
What company, at what expense; and finding,
By this encompassment and drift of question,
That they do know my son, come you more nearer
Than your particular demands will touch it:
Take you, as 'twere, some distant knowledge of him;
As thus, 'I know his father and his friends,
And in part hi;m;--do you mark this, Reynaldo?

Reynaldo
Ay, very well, my lord.

Polonius
'And in part him;--but,' you may say, 'not well:
But if't be he I mean, he's very wild;
Addicted so and so;' and there put on him
What forgeries you please; marry, none so rank
As may dishonour him; take heed of that;
But, sir, such wanton, wild, and usual slips
As are companions noted and most known
To youth and liberty.

Reynaldo
As gaming, my lord.

Polonius
Ay, or drinking, fencing, swearing, quarrelling,
Drabbing:--you may go so far.

Reynaldo
My lord, that would dishonour him.

Polonius
Faith, no; as you may season it in the charge.
You must not put another scandal on him,
That he is open to incontinency;
That's not my meaning: but breathe his faults so quaintly
That they may seem the taints of liberty;
The flash and outbreak of a fiery mind;
A savageness in unreclaimed blood,
Of general assault.

Reynaldo
But, my good lord,--

Polonius
Wherefore should you do this?

Reynaldo
Ay, my lord,
I would know that.

Polonius
Marry, sir, here's my drift;
And I believe it is a fetch of warrant:
You laying these slight sullies on my son
As 'twere a thing a little soil'd i' the working,
Mark you,
Your party in converse, him you would sound,
Having ever seen in the prenominate crimes
The youth you breathe of guilty, be assur'd
He closes with you in this consequence;
'Good sir,' or so; or 'friend,' or 'gentleman'--
According to the phrase or the addition
Of man and country.

Reynaldo
Very good, my lord.

Polonius
And then, sir, does he this,--he does--What was I about to say?--
By the mass, I was about to say something:--Where did I leave?

Reynaldo
At 'closes in the consequence,' at 'friend or so,' and
gentleman.'

Polonius
At--closes in the consequence'--ay, marry!
He closes with you thus:--'I know the gentleman;
I saw him yesterday, or t'other day,
Or then, or then; with such, or such; and, as you say,
There was he gaming; there o'ertook in's rouse;
There falling out at tennis': or perchance,
'I saw him enter such a house of sale,'--
Videlicet, a brothel,--or so forth.--
See you now;
Your bait of falsehood takes this carp of truth:
And thus do we of wisdom and of reach,
With windlaces, and with assays of bias,
By indirections find directions out:
So, by my former lecture and advice,
Shall you my son. You have me, have you not?

Reynaldo
My lord, I have.

Polonius
God b' wi' you, fare you well.

Reynaldo
Good my lord!

Polonius
Observe his inclination in yourself.

Reynaldo
I shall, my lord.

Polonius
And let him ply his music.

Reynaldo
Well, my lord.

Polonius
Farewell!
[Exit Reynaldo.]
[Enter Ophelia.]
Warning: Missing line number 0

Polonius
How now, Ophelia! what's the matter?

Ophelia
Alas, my lord, I have been so affrighted!

Polonius
With what, i' the name of God?

Ophelia
My lord, as I was sewing in my chamber,
Lord Hamlet,--with his doublet all unbrac'd;
No hat upon his head; his stockings foul'd,
Ungart'red, and down-gyved to his ankle;
Pale as his shirt; his knees knocking each other;
And with a look so piteous in purport
As if he had been loosed out of hell
To speak of horrors,--he comes before me.

Polonius
Mad for thy love?

Ophelia
My lord, I do not know;
But truly I do fear it.

Polonius
What said he?

Ophelia
He took me by the wrist, and held me hard;
Then goes he to the length of all his arm;
And with his other hand thus o'er his brow,
He falls to such perusal of my face
As he would draw it. Long stay'd he so;
At last,--a little shaking of mine arm,
And thrice his head thus waving up and down,--
He rais'd a sigh so piteous and profound
As it did seem to shatter all his bulk
And end his being: that done, he lets me go:
And, with his head over his shoulder turn'd
He seem'd to find his way without his eyes;
For out o' doors he went without their help,
And to the last bended their light on me.

Polonius
Come, go with me: I will go seek the king.
This is the very ecstasy of love;
Whose violent property fordoes itself,
And leads the will to desperate undertakings,
As oft as any passion under heaven
That does afflict our natures. I am sorry,--
What, have you given him any hard words of late?

Ophelia
No, my good lord; but, as you did command,
I did repel his letters and denied
His access to me.

Polonius
That hath made him mad.
I am sorry that with better heed and judgment
I had not quoted him: I fear'd he did but trifle,
And meant to wreck thee; but beshrew my jealousy!
It seems it as proper to our age
To cast beyond ourselves in our opinions
As it is common for the younger sort
To lack discretion. Come, go we to the king:
This must be known; which, being kept close, might move
More grief to hide than hate to utter love.
[Exit Ophelia.]
[Exit Polonius.]
Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare
[Enter King.]
[Enter Queen.]
[Enter Rosencrantz.]
[Enter Guildenstern.]
Warning: Missing line number 0

King
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
The need we have to use you did provoke
Our hasty sending. Something have you heard
Of Hamlet's transformation; so I call it,
Since nor the exterior nor the inward man
Resembles that it was. What it should be,
More than his father's death, that thus hath put him
So much from the understanding of himself,
I cannot dream of: I entreat you both
That, being of so young days brought up with him,
And since so neighbour'd to his youth and humour,
That you vouchsafe your rest here in our court
Some little time: so by your companies
To draw him on to pleasures, and to gather,
So much as from occasion you may glean,
Whether aught, to us unknown, afflicts him thus,
That, open'd, lies within our remedy.

Queen
Good gentlemen, he hath much talk'd of you,
And sure I am two men there are not living
To whom he more adheres. If it will please you
To show us so much gentry and good-will
As to expend your time with us awhile,
For the supply and profit of our hope,
Your visitation shall receive such thanks
As fits a king's remembrance.

Rosencrantz
Both your majesties
Might, by the sovereign power you have of us,
Put your dread pleasures more into command
Than to entreaty.

Guildenstern
We both obey,
And here give up ourselves, in the full bent,
To lay our service freely at your feet,
To be commanded.

King
Thanks, Rosencrantz and gentle Guildenstern.

Queen
Thanks, Guildenstern and gentle Rosencrantz:
And I beseech you instantly to visit
My too-much-changed son.--Go, some of you,
And bring these gentlemen where Hamlet is.

Guildenstern
Heavens make our presence and our practices
Pleasant and helpful to him!

Queen
Ay, amen!
[Exit Guildenstern.]
[Exit Rosencrantz.]
[Exit Queen.]
[Exit King.]
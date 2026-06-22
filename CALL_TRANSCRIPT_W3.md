Yukihiro <> Max - Rustify - June 22
VIEW RECORDING - 70 mins (No highlights): https://fathom.video/share/exHPmmxMv-VKwvkB5cvs2ywvTmPGpu2-

---

0:00 - Yukihiro Taira (Rustify.rs)
  Good morning. Good morning, how are you? Better. I'm not sick anymore, so that's great. Okay, cool. Sorry, I did see that it is on my calendar, and I have it, sorry.  Okay, okay.

0:15 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, no worries, no worries. Okay, so you have seen that I've pushed some changes in the application.

0:26 - Yukihiro Taira (Rustify.rs)
  Yeah, I was actually looking at it. I'm sorry to interrupt. I was actually looking at it right now, and so I'm trying to find a file that is hg64, because I have a lot of QuickTime stuff.  Oh, this is hg64. Let's see. So I'm getting an error of 4.2.2, fail to launch. 4.2.2, oh wait, maybe because it's broadcast?

0:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe I've done some stuff wrong. If you want, you can share your screen also if want. Okay, let's see.

1:04 - Yukihiro Taira (Rustify.rs)
  Share screen, share entire screen, this one share. Cool. I just have this in, you know, like, the VS Code thing.

1:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, let's see. I don't have this kind of error, so fail to launch FFFF. Okay. Yeah.

1:34 - Yukihiro Taira (Rustify.rs)
  Yeah.

1:35 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Can you maybe restart, stop the server and restart, maybe?

1:40 - Yukihiro Taira (Rustify.rs)
  Let me put this here. I'll put this here. Let's see. Yeah, maybe also I've done some changes in the data.

2:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  The structure, I don't remember for this, but maybe this explains the error, I don't know, let's see. Okay, what changes did you create?  I've made, you know, like there is a plan, a plan video debugger pivot in the, yes, in the plan, in the MDFI here.  Mainly this changes, so the idea was mainly to show if all is good, the warning, and stone when you drop a file.  I've also added some video files also with specifically some errors and stuff in them to make sure it was working.  Okay. So if you go, for example, in the, it's, uh, You have a test folder in the root. In here?  Yeah, in the root, you have a test folder. Test folder, let's see.

3:18 - Yukihiro Taira (Rustify.rs)
  Test, there we go. You have the stuff in here. Oh, I see, I see, I see. Okay, let's try these.

3:30 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe this way it will not work. Maybe you need to do this from the, yes, exactly.

3:39 - Yukihiro Taira (Rustify.rs)
  Okay, that's weird. Okay, excuse me.

3:45 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. And can you just, in case, cut the server and do a reset DB? You know, like this. Maybe this is the one.

4:05 - Yukihiro Taira (Rustify.rs)
  Right here? This one right here? Yes, this one, sorry.

4:12 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Oh yeah, I did that. Oh, SH, yeah, yeah.

4:16 - Yukihiro Taira (Rustify.rs)
  Yeah, this one.

4:18 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so it should be good. And now if you run again DX itself, hopefully it will be better, because when I tested it, I didn't have any issues.  So, yeah, so let's see. Because FFProb, is it something that we need to install? I don't remember how I did it.  Maybe it's, yeah, but... I'm actually not sure, to be honest.

4:54 - Yukihiro Taira (Rustify.rs)
  I'm kind of still learning as you're learning, so... Yeah, Yeah, maybe it's something that you...

5:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  ...that we need to install for FFprop. I don't remember because it's still new also for me. yes, here it's roots, at, yes.

5:16 - Yukihiro Taira (Rustify.rs)
  password, diagnostic.

5:26 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, that's fair. Can you try to just copy-paste the errors that you have in cloud code or something like this?  Maybe it's because you didn't have FFprop on your machine, maybe? Ah, maybe that is.

5:45 - Yukihiro Taira (Rustify.rs)
  Yes.

5:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe, I don't know if you use cloud code directly in your terminal with your machine, so it has access also to what is installed in your machine and so on.  I don't if you do this or not.

5:57 - Yukihiro Taira (Rustify.rs)
  Oh, I just use it... How is it not? It's not copying. Interesting. Copying. I use it in the actual app, but I should probably install it on the command line, like you said.

6:13 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, because I don't know if it has the... Because on the app, does it have access to your machine with all the stuff that you have installed on your machine and so on?

6:24 - Yukihiro Taira (Rustify.rs)
  Um, I believe I have it as much as it can, but I don't have it... I don't know if it's looking at the root.  Okay, okay. Yeah. Because I haven't been using it in that way. I literally just, like I'm doing right now, I would put it in an error and see what I get, because I don't want to be fully reliant on it, but I probably should do that, yeah.

6:48 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It's more like to help you understand faster. It's more like this. I yeah.

6:54 - Yukihiro Taira (Rustify.rs)
  But here, probably... I don't... I don't... it says I don't have FFMPEG. Okay. Um, probably... Probably. It's a soluble process of by Verified, so I should probably do ffmpeg, right?

7:08 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, probably, and I guess also if you do ffprobe...version, it will...

7:18 - Yukihiro Taira (Rustify.rs)
  Should I do ffprobe?

7:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, just to check the version, because I guess that if you do ffprobe...version, it will put nothing or something like this.  I guess it will have nothing, dash dash version. It has one dash, but okay.

7:38 - Yukihiro Taira (Rustify.rs)
  it's two dashes. Not found, so I probably have to install ffprobe, too.

7:43 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, probably, also.

7:45 - Yukihiro Taira (Rustify.rs)
  Let me do the ffmpeg, because I want that to install ffmpeg.

7:57 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And for ffprobe, don't remember... If it's something that you also need to install, or if it's already included when you do FFmpeg, I would check with me what I have with FFprobe.

8:26 - Yukihiro Taira (Rustify.rs)
  So I did full includes additional tools. So I have FFprobe 8.1.2 right now. Missing argument option version. That's weird.  It gave me the version, and then it gives me an argument of missing version. Yes, that's weird. Yes, that's weird.  It's working here, but I don't know. Yes.

8:54 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, at least we have FFmpeg, so if you restart the server, hopefully it will work now. It's a thing for me, when I try this, we have an issue.

9:23 - Yukihiro Taira (Rustify.rs)
  Yeah, I've been, as you said, you're busy, I've been very busy too, and I've been trying to find time to juggle things right now.  And yeah, I'm a little behind on everything, but I guess this is something that I need to battle, you know?  Yeah, yeah, yeah, no either.

9:41 - Maxime Montfort (maximemontfort.pro@gmail.com)
  There we go. Oh, that's perfect.

9:45 - Yukihiro Taira (Rustify.rs)
  So I do want to test this right here with subtitles, because I do have a test file that has subtitles.  Okay, one fail, video codec ProRes. So ProRes. No subtitles, let's see what I... Okay, maybe I did something wrong with the...  No, no, no, no, no, no. Subtitles are the most difficult thing to embed in a file. Like, literally, it's so difficult.  So that's why I have a lot of... What is this? I have a lot of, like, caption files that are, like, what is this?  DaVinci Resolve. Oh, this is HDR. Let's see what this does. This is going to take a while. Oh, I can't do this one.  That makes sense. What is it? HDR. So HDR is a 10-bit file. It's actually 16-bit, but you throw away the 4-bits.  It's a 10-bit... So usually a video file is 8-bit. So when we're talking about, you know, Rec. 709 or...  Anything you see that is normal content will be 8 bits, but HDR adds two bits to the luminancy, so how bright the screen can get.  So you know like on your phone when you buy a phone lately, they're like, this goes up to, I think this goes up to 1600 nits.

11:18 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay.

11:19 - Yukihiro Taira (Rustify.rs)
  So nits are the light projecting out of the screen. Okay. There's a lot of light measurements, but a nits is the projection out.  Okay. So it's a 1600 nits would basically mean that it can do, you know, 1600 of luminancy. A normal screen is 100, and HDR is anything from 400.  Okay. So, and the ideal Dolby, so when you go to the movie theater and it's Dolby Vision, then that is going to be 10,000 nits.  So let me just show It's easier to visualize it. I'm not going to make it too long today, so...

12:06 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, because I'm a big noob in everything related to video, so... Oh, you know.

12:15 - Yukihiro Taira (Rustify.rs)
  This is very advanced stuff. This is like new stuff, anyways. So, let's see. What's a good one? Something easy to understand.  I mean, this is the PQ curve.

12:29 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. I don't know, which is not there.

12:33 - Yukihiro Taira (Rustify.rs)
  I hate it when you click into it and it's gone. here.

12:36 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It was here. It was in the... If you go back, it was in the page made in very detail.

12:43 - Yukihiro Taira (Rustify.rs)
  What was it? Was it in there? Oh, there it is.

12:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, yeah, Maybe just to... So, the PQ operating point, air flow rate.

12:52 - Yukihiro Taira (Rustify.rs)
  Okay, so this is static pressure. So, yeah, this is hard to understand, but basically... The static pressure is the, do you know anything about photons and, like, meshes?  Yeah, so this is a quantum physics thing where you have the, I think it's called the bisone mesh or something.  And there's four meshes in our reality, and depending on the size of the particle, it is able to penetrate.  So if it cannot penetrate, it adds mass to the molecule. If it penetrates, it goes through that, and then there's no mass.  So a photon is something that has zero mass, means that it goes through all the meshes. And this is why we can look at a universe that is 50 light years away from us, and the color information, the light information, comes in at the same time.  So when you're looking at a galaxy 50 light years away, it's not like the information. is coming back late.  Every single pixel, every single color is coming out back at the same. So this means that the speed of light is consistent throughout the space continuum.  So that is the static pressure of each molecule going through space and then the rate of the airflow. So this is a very scientific thing.  There's an easier way to look at it. Oh, I feel like there is a, at my old company, I had a nice TQ curve.  Oh, there we go. What about this one? I mean, just for the sake of this conversation, this might be fine.  Okay. So luminance value, 10,000. Let me zoom in a little bit. So this is the luminance value. So that's 10,000.  Usually our phones are, they're around like a thousand. Most three. We can only do up to 2,000, maybe 3,000.  As humans, we have not created this yet on a screen. You can do it with projectors because it's just the amount of light, but you can't do it on an actual physical screen.  So the PQ curve is basically, this is the value of the highlights, the 80% point on the highlights. So when you're looking at an image, the parts that are bright are flat compared to other stuff.  So this is all the low end, and you have the mid-tones, skin tones, blah, blah, blah, blah, blah. So the PQ curve is if the target luminance is here, then at the most brightest point of the image, it's going to boost that to this bright.  So let's say your skin tone, which is usually neutral tone, is at 500. So see how it's basically not boosting anything?  But then when we get into 700, 750, which is very light, bright, like if I were to hold up, you know, like a screen and it made it very bright.  Well, this is the cameras changing it. So let's see. Oh, let me put up a flashlight. Flashlight it cannot deal with.  So that's to see how it's a white. It's just white. There's no information. So in HDR, it will attempt to make this white light.  It will push this bright light up to here so that this part under this right here can produce color.  This part here, usually this would be a straight line, but this by pushing this up, you're adding bandwidth here.  light Thank And you can produce color within that. So this is basically what HDR is. It's producing color in the highlights.  Anything else stays the same, but the highlights get pushed up so much that you have more resolution in the color rendering.  And it's also brighter.

17:21 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And this is something that you use a lot when you work with videos? This is the next frontier.

17:30 - Yukihiro Taira (Rustify.rs)
  So this is why when you buy a phone now, the new phone is like 2000 nits. Or when you buy a laptop, you can go up.  This is an Apple thing because Apple is pushing HDR hard. Usually when you buy a Windows computer monitor and it says HDR, it could be like 400.  But 400 is not true HDR. True HDR is 10,000. So Apple is striving, working with Dolby because every Apple product has a Dolby license.  So if you have an Apple product, then you already have a Dolby-enabled device. So they're trying to push their devices or their hardware to accommodate for the Dolby standard, because PQ is Dolby.  And when you say HDR, and I think there's three main standards, there's the Samsung one, and then there's the Dolby one, and then there's the normalized American HDR one that is used.  If you're not using either one of those, it would be that one. And right now, I think Dolby is winning, because when you go to cinema, you have the Dolby Vision, you know, that they have that.  And a lot of devices now are now using, you know, there's a HDR10 and 10 plus, that's the Samsung one.  Um. And you see more of Dolby. I don't know, maybe in France it's different, but in America, you see more Dolby than you see HDR10 now.

19:07 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, I didn't notice, to be honest, but okay, okay, okay.

19:12 - Yukihiro Taira (Rustify.rs)
  Yeah, so there's a fight, you know, it's kind of like everything where, you know, oh, this is the new standard, this is the new standard.  Dolby is trying to fight Samsung and everybody to get their standard out there. And this is a more capable mathematical standard.  However, it is in theory and it's very difficult. So the only way to get there is if all the hardware around it can actually get to 10,000 nits.  And right now, no screen, like if we look up what is the Apple, the iPhone is going to have the most because it's a phone.  And 18, it's going to be. you. So 3,000 nicks at peak. So if we go back, so yeah, 3,000 nicks, the typical HDR Max is going to be 1,000 nicks or up to 1,600 nicks.  So when we go back to this curve here, oh wait, where was it? So that's going to be around here.  So the new iPhone that's coming up this year is going to be here. Okay. But usually it's going to be operating at here.  So when you're looking at the phone, this is what you're going to see. We're still very far away from actually physically creating 10,000 nicks.  And that's why it's the next thing. So when you ask, is this something that we use a lot? Not really.  However, it is something that is currently getting more adopted and, you know, manufacturers year by year are making sure that we are going to hit this target of 10,000 nicks.  . Okay.

20:58 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. first thing, okay. . Yeah, so I would just look at it.

21:02 - Yukihiro Taira (Rustify.rs)
  It's a very difficult thing to do, also. It also requires tonemapping, so it pushes...let's see if I can find...HDR.

21:15 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And for HDR, does it, I don't know, does FFmpeg, for instance, is able to extract, I don't know, this value from a video, for example, or is it really more, like, related to hardware?

21:36 - Yukihiro Taira (Rustify.rs)
  So, yes, so it will be able to at least get HDR, you know, is it HDR or not? Will it be able to see the tone mapping values?  That I do not know. Okay. I don't think that for the uses of FFmpeg right now, HDR is on their top priority.  Yeah, I guess. Uh. Mhm. that, But it will be something eventually put in, and that I do need to look into, but the problem is that, so here you have Rec.709, which has been the standard since the 90s, this is the standard that Microsoft, Apple have adopted, and most screens operate at Rec.709, and you have DCI-P3, so like this, both of my screens right now have DCI-P3 100%, and this is a, anything platform, so basically you see how it has a thousand nits right here, it's just under a thousand nits of color reproduction, and then you have a thing called Rec.20.20, so Rec.20.20 is a theoretic amount of color that our eyes can see, so that is a mathematical scientific value, which no screen, nothing has produced that yet, and as far as humans know, we haven't been able to produce a screen  The only use case right now is the, basically, it is to edit things, so when you're editing, shooting something, you shoot in 2020, and then you color create it and all that, and so you can push colors here and there, and then you close it down to Rec.  709, so that, you know, you can show it So this is Rec. 2020 here, this outside color, this is the full, this is what your eye can see.  Okay. So this whole, all this color here is what your eye can see, and then this triangle here is Rec.  2020. Okay. Basically, we're cutting off a lot of greens, a lot of this ultraviolet stuff, some of the infrared gets cut off, but most of this is, most people can see within this range.  Some people can see it, hear some people can We'll see more here. It's known that women can see more reds than men, but it depends on the person.  So the REX-2020 is achieving the maximum value overall of human beings. This inside one is REX-709. So this is our current color space that we use on 99.9% screens.  It's right here. So this here is this difference here. So this is where that 10,000 nits is going to be.  This color value. Right now, what happens if you show this onto a screen, it will push this information into this triangle.  And it will become a much. It will look flat. So by making the screen brighter, you'll be able to reproduce this curve here.  And I know you probably cannot see it on your screen because it's getting compressed. But if you look at this image on your own screen, you can see from here.  In here, it's a completely different color.

25:02 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, maybe, yeah, for me, it looks a little similar.

25:06 - Yukihiro Taira (Rustify.rs)
  Yeah, it kind of looks like the same color, probably, because it's pushing it, it's doing that. That's, so this is the FFMP part, it is taking this value, and it's pushing it into here, so that on the other side, you cannot see that information.  The HDR is trying to achieve a solution to that, and so we don't have to push it in here, and we can actually produce, like this is very green, this is kind of like bluish green right here, but you can't see that when you push it through a process.  Okay, okay, interesting.

25:42 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah.

25:43 - Yukihiro Taira (Rustify.rs)
  Sorry, I always over-explain, I'm sorry.

25:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  That's interesting, and do you think, so in the applications that we build, yeah, guess having information about HDR would be maybe interesting.  Or maybe to see the compatibility with HDR from a video maybe. I guess this is possible with FFmpeg. For now, I don't think it's important really.

26:16 - Yukihiro Taira (Rustify.rs)
  I think getting the working thing and then we can add that later.

26:21 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And from what you see then, what do you think is important? What do you see next? Because at the moment it's just running the diagnostic to make sure that everything is good and so on.  But what do you see next once we have done the diagnostic from the video?

26:45 - Yukihiro Taira (Rustify.rs)
  Yeah, so there is a file on, was it this? Yeah, so this person, it's called MP4 Analyzer. So this person has a good...  So something like this is really where I want to go. So this is done in Python, which we all know is not the fastest thing to do this type of thing.  Let's see if I can zoom in.

27:17 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So this is giving us frame data.

27:20 - Yukihiro Taira (Rustify.rs)
  So this is the 240th frame, 241, each frame. It also is giving us the size, the color matrix, you know, all of that information, duration, audio tracks.  There's two tracks, file size, the bit rate. So something like this. And I believe I haven't compiled this at all.  I haven't opened it up. I believe this plays too. So as it plays, it gives you the information on the data.  You can see there's a red flag here on this frame. And I, I believe, oh yeah, right here. Playback control.  So you can scrub through it. Okay. So this is. Just one more thing. Why this is important is when you're looking at this frame, I believe this is going to be an I-frame.  So in I-frames, B-frames, you know, there's a couple different types of frames, but the important ones is I-frame is a full frame.  It's a photo. And in between, so there's one, two, three frames, and then you have an I-frame. So basically what the encoder is doing, it takes these two frames and it calculates what it thinks is correct in between.  So this is not real data.

28:37 - Maxime Montfort (maximemontfort.pro@gmail.com)
  The actual data is here.

28:39 - Yukihiro Taira (Rustify.rs)
  So when we're doing compression or when we're trying to compress something, the amount of I-frames will determine the file size.  It will determine the quality of the data. It will also determine how accurate the in-between files are going to be when played back.  week. So I'll get So So having a tool that shows you where the iframes are is very important. Okay, I see.

29:08 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And also the red thing, what is it? don't remember. What kind of error is it? This era?

29:17 - Yukihiro Taira (Rustify.rs)
  I'm not really sure, but I believe maybe this era might be saying that this is an incomplete frame. Okay.

29:25 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, it's possible that in a video the frames are not good. Like, I don't know, like, maybe it's a naive question, but let's say that you are making a video for YouTube, for example, with a camera that is filming you.  Maybe it's possible that, I don't know, in the camera there is a little, maybe it's very hot and there is a little bug in the footage.  And this would be the frame error, for example? Yes, and most...

30:00 - Yukihiro Taira (Rustify.rs)
  In publications of video files, they kind of throw away frames, so if it's not a good frame, it'll just throw it away and interpolate it, so what happens is, you know, it depends, the reason why I work in ProRes, so ProRes is an Apple format that is made for Final Cut, it's made to edit, so you're supposed to go frame by frame, but if you're just watching an MOV, not an MP4, like a normal video, just because the frame cuts out, doesn't, it doesn't break the experience, because if you're doing, I don't know, 30 frames per second, you're cutting up one second into 30 different slices, and for the most person to see, looking at the screen, and they see one frame cut out, it's not that important, obviously, if you cut up like five, six frames, then it's like, oh, the screen went black, but because it's so fast, you know, one frame getting cut out is not,  What's important is the cadence. So how consistently am I sending frames at you is more important because you don't want it so that it goes fast and then it goes slow.  You don't want that. You want it to be consistently at the same speed. So it's either are we going to make sure that the cadence is properly or are you going to make sure that each frame is complete?  And each frame complete is really ideal, but the cadence is way more important. So they focus on more cadence and how can we serve the data consistently?  And that's how they're structured. But then ProRes or like there's certain, you know, formats out there that are, you know, it's not about the consistency.  It's about the frame data and how here is the data. So in ProRes, there's no interpolation. It doesn't guess.  If you don't have a frame, you don't have a frame, because it's made for editing, and it's made for, you know, production, so ProRes files are extremely big, for like a minute it's like 10 gigs, you know, it's extremely big, for that reason, because each frame is a full frame, has all the information, and so to be able to play ProRes on a device, means that you need to have extremely fast memory, and that's why Apple has focused on memory speed on their devices more than anything, because their format supports that.  Okay, interesting.

32:39 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, okay, okay. And just also, I was wondering, for example, in this example, so we see, I don't know, maybe 100 frames or something like this, is it, like, is it, I guess it's a short video, like, I'm asking naive questions, but just ringing for the time.

32:59 - Yukihiro Taira (Rustify.rs)
  Yeah.

33:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So maybe a video of 10 seconds regarding the frames. Or maybe we don't see all the frames. I'm trying to understand how it's rendered if you want to see, for example, a video of two minutes for these details of frames.  I guess it would be a lot of frames to render.

33:24 - Yukihiro Taira (Rustify.rs)
  It depends. So this right here, let's see if it says ABC1 sample frame rate timescale. So it's at 30 frames per second.  So every 30 frames is going to be one second. So if we just do 901 divided by 30, that means this is a 30 second clip.

33:52 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay.

33:53 - Yukihiro Taira (Rustify.rs)
  Yeah. So this whole bar from the zero value to 100 is going to be almost right. Come This Well, I guess 99 is going to be 30 seconds in time.  But we don't really use time in video. That's the thing, because absolute time is time, which is not the scale of what you're actually trying to play.  We use frame time. Okay. I remember you told me that last time. Yeah.

34:24 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I think you told me that sometimes we do the kind of conversion between time frame and absolute time when there is some differences between the two.  Yes.

34:40 - Yukihiro Taira (Rustify.rs)
  Yes. So each facility, well, the facility that I worked at, we had a time code generator, and we had two of them.  One was generating time of day, and the other one we are generating time code. Um, and you would send both of those to our encoders, um, with a SDI.  And the encoders that we were creating there had the capability to grab both. Okay.

35:10 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So, yeah.

35:11 - Yukihiro Taira (Rustify.rs)
  And, you know, time of day is something that is, you know, it's digital now and it's calculated by the revolutions of the earth and, you know, standby satellites and all that.  But that necessarily doesn't mean that the time of day on your computer is synced to that. You could be offline, you know, and you could be so far away.  So like right now with, I think on iPhones, it uses the Unix, well, yeah, Unix time base and the Unix time base, it was created in 70, 1970 something.  So when you, yeah, when you reset this and it doesn't have access to the internet. It's like, oh, I'm going to restart in 50 years because it thinks it's at 1970 or something.  So you have the start point and then it's calculating how far we are from the start point. So that's how computers are trying to do things.  So if you have the appropriate time of day through the Internet, it's fine. But when you don't have that, then you start to see a lot of differences in systems.  And this is where this type of diagnostic tool is very important because it needs to be consistent. There's never a professional environment where you only have one of everything.  There's always at least two of everything, two cameras. Even if you're using one camera, you have two because what if the other one dies?  You know, or, you know, if we're going to record something, always record two things, two of the same thing.  Because you never know what's going to happen. Somebody might spill a drink on one of them and then we're out, you know, so you never know what's going happen.

37:11 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, so there's always going to be two of everything.

37:13 - Yukihiro Taira (Rustify.rs)
  So how do we sync that information? How do we make sure that this thing and this thing are doing exactly the same thing?

37:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And that's why you go by frame data.

37:25 - Yukihiro Taira (Rustify.rs)
  Okay, okay.

37:25 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, makes sense. And okay, because so also in the what you show, so I'm thinking about how we can improve the application.  So here, for example, we would, because at the moment, in the application, we dropped the file, but we just see the information.  Probably it would be better also to have, as in here.

37:51 - Yukihiro Taira (Rustify.rs)
  So yeah, this is what we're saying right here on the left side. Yeah, yeah, yeah.

37:59 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, and Maybe there are some information also we see here that we don't see in the application, in the Dioxus application, but from your perspective, I guess for the next step, the most important would be to be able to render the video and to click on the different frames, for example, I guess.

38:30 - Yukihiro Taira (Rustify.rs)
  Well, to be more realistic, since, you know, we're not like a multi-million dollar company or anything, I think the most important thing is to get more information, get as much as possible, every single thing.

38:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  What information is missing, for example, here? What information is important? The rotation.

38:51 - Yukihiro Taira (Rustify.rs)
  So, interesting, it's grabbing rotation here, because this is an apple variable. Rotation is... Is the camera rotated or not?  Yes, yes, yes. So it's a phone thing. We have AAC. So here, so AAC codec, what is, okay, I guess it has sample right here.  And then AAC mp3, death not applicable, default audio stream, default flag set player, we'll select this show, okay?

39:21 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, I'm afraid to put as much information as possible, but I don't know what is important, actually, and what is missing, also, yeah.

39:31 - Yukihiro Taira (Rustify.rs)
  I mean, everything here is important. Variable frame rate is good, you know, you want to know if it's variable or fixed.  This is, everything here is nice. It would be nice to see more, though. You did have that one file, where was it?  It was in to-do or something?

39:54 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Or what is that? Inspect. Maybe, I don't remember. Was it in spec?

40:04 - Yukihiro Taira (Rustify.rs)
  No, maybe not. Oh, no, it was in GitHub. You had it in... Oh, and I did this. Oh, that's good.  Perfect. Awesome.

40:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe you could complete a little more about the output section and everything, but that's very good.

40:27 - Yukihiro Taira (Rustify.rs)
  I need to complete more, yeah. I just was, yeah. You had it in issues in here.

40:37 - Maxime Montfort (maximemontfort.pro@gmail.com)
  The JSON. Ah, yes, yes, yes. Okay, yeah, yeah, I remember that. Okay, okay. Maybe...

40:44 - Yukihiro Taira (Rustify.rs)
  Yeah, so all of this is actually very important, actually. That's why it's hard to say we don't need anything, because everything is relevant here.  Although... Clean effects, like disposition. This stuff, not really. I mean, it's a disposition, so if there is something there, then it is important.  But right now it's a default, so none of this will be one. Everything else will be zero, and it's going to be on default.  And I think it's, this is also, so if there's like descriptions, this will be one, and this will be zero.  So it is important, but most things are going to be on default, so not really that important. But I mean, sample aspect ratio, we want to know if it's one by one, because are we using rectangles, or are we using triangles?  This means we're using squares, which most things are. So this is 30 frames per second, you know, 31 by 30.  progressive, has two B frames. So this is what I was talking about with the B frames. And we're done.  You So you have one iframe, two b-frames, another iframe.

42:04 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. So that's going back to the other... For example, because here we are doing a kind of diagnostic, so is it possible that in the reason you were showing that in a specific video file, some data are wrong?

42:29 - Yukihiro Taira (Rustify.rs)
  Yes, there is a possibility that it is not reading it correctly, and it is, yeah, so that's testing, you know, it just has to test a lot of different softwares and going through different pipelines just to see how correct it is.  A good software, actually. You have to pay for this one, though. Video... A good software, actually. tool, tool, exr, tool, why is it not showing up?  vmix, exr, video, tool, open exr, viewer, view. I don't know why I forgot. I used to use this every day, and now I'm like forgetting.  It's just not the exr. Yeah, maybe you will find it.

43:47 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And also, yes, by the way, before I forget, also, if you can send me the link of, you know, the Python things that you showed me on GitHub.

43:58 - Yukihiro Taira (Rustify.rs)
  This one right here. Yes, this one, yes. Okay.

44:01 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, this way I can, it's a good reference. I think I can just, yeah, just the link so I can check.  I'll send it on WhatsApp. Yes, perfect, perfect. And also if you, because you seem to do a lot of research and that's perfect, if you find some interesting stuff like this, you can also put in the project or anything.  This way we don't forget also, because this is very useful.

44:42 - Yukihiro Taira (Rustify.rs)
  Okay, where should I put it?

44:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  You can create, I don't know, a new MD file with, I don't know, inspiration.md or something like this. And then all the relevant projects that you see.  Oh, okay.

45:02 - Yukihiro Taira (Rustify.rs)
  Yeah, okay. I got it. Okay, perfect. And then I just wanted to find that one thing, but I don't want to take too much time.  But it was really cool. Where did I find it? It was like the first thing that showed up the other day.  And I was like, oh, I can easily find that. And now here I am. Oh, maybe for this, let's say you can put in the MD file to make sure to not forget to find it again.  Yeah, I found it. It's called MediaInfo.

45:37 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. There you go. MediaRena's MediaInfo.

45:41 - Yukihiro Taira (Rustify.rs)
  So I used to use this a lot. Let's download MediaInfo. Open a Mac store. Let me just download it right now.  It should be pretty fast. It's a small file. Open. Cool. Oh, I used to use it so well, because this is really good, so what you do is you just open a file, actually, what was the folder that we had, this one, and just drop that in, and let's do here, so this gives you all of the information that this sees.  Okay. So it's a constant, yeah, and then 720p, then it's video-coder-contained baseline, three reference frames, it gives you, just basically, this is a nicer way to look at it, all the information, you know, your audio, you have a stereo audio right now, because it would give me ID1, ID2, if it was mono, so this means it's stereo, audio, it's channel one, one channel, it's a mono file, but that doesn't.  Meaning, it's a mono file. It doesn't have a stereo output. It's either mono or it's a stereo mono, which means that it's giving you two of the same thing.  Let's see, this one right here was this file extension mismatch.

47:17 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And it always distinguishes between video, audio information, and general stuff. It's always divided into three categories. Not necessarily.

47:31 - Yukihiro Taira (Rustify.rs)
  If there's another category, like subtitles, it will give you another subtitle. But this, yeah, I mean, this is the main thing, the general stuff, the stuff that every single file needs, you know, duration, bitrate, frame.  Like, if you don't have this information, you're not playing a file. And then you have, because sometimes you don't have a video file.  Sometimes it's just an audio file. And then I'll show the audio. But yeah, video. Okay, okay.

48:04 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And because in the application that we have with the Auxys, at the moment, it was mostly kind of diagnostic, but you had to display also the general information, maybe other general information, I don't know.

48:19 - Yukihiro Taira (Rustify.rs)
  So, I think it was this one here, Media Read, I think it was? No. was all the JSON info.

48:34 - Maxime Montfort (maximemontfort.pro@gmail.com)
  This one right here. So this one is using some type of playback? Yes, but it's using web, you know, like when you have the three dots here?  Yeah, yeah. Using web, so it's, you don't have all the, it's not as good as the Python stuff, I guess.  You cannot see all the frames and so on. Yeah, it's, but...

49:00 - Yukihiro Taira (Rustify.rs)
  In the beginning, I was going to say, like, for this, like, when you drop something in, I mean, if this could be that, at least, you know, or like a bigger version, because this is pretty small here, you know, and this, for some reason, full screen is probably toggled off, so we have to toggle that on.  I haven't looked into the code base at all, so I don't know, but this, you can't go full screen.  So maybe, what I really like as a feature, actually, is, you know, like in YouTube, they have cinema mode, where it expands it to the edge of the thing, but you still have it in, you can still see the comments, but it just expands the video, you know, from here to here, and then you have the comments.  Having something like that would be nice, so when you drop something in. Okay, okay, okay. You know?

49:54 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And also, yes, by the way, if you have, for example, something like this, where you... Where you see a design that is cool.  Also, can put it in the project. This way, I will check this also. will help. But yeah, so when you put several videos at the same time, you mean?  Oh wait, what is this showing?

50:18 - Yukihiro Taira (Rustify.rs)
  Oh, I just dropped it. Let's see. Let's see. What happens when we put several in there? I haven't tried that yet.  I'm not sure.

50:27 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Oh, it only does one.

50:29 - Yukihiro Taira (Rustify.rs)
  It only did the top one.

50:30 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, I didn't handle this yet. Yeah. But this could be cool also to handle, you know, like, mucus reading stuff and so on to do in parallel.

50:44 - Yukihiro Taira (Rustify.rs)
  Yeah, that would be interesting. So like, on my computer here, let's see if we go here, new. If you go, let's see, document.  This document. It's like, I hate how they split their file system and now my whole computer is just messed up.  Um, I guess it's, it's somewhere, I just need to document, no, here, there you go. Okay, so we have, let's see if there's one that I have both, it says download this one.  I was hoping that I can find something where, like a working project, so in a working project, you would have multiple files of the same thing, just because, okay, we need to add, oh yeah, ooh, that was heavy, so if you can see here, this is an Ed Sheeran show, and let me just do this is Yuki, DaVinci, you know, blah, blah, blah, blah, blah, ProXQ, 178, P3, D6, C5, so this means it's HDR color format, and then I have a Redo, and then I have a Premiere Pro, because this is DaVinci Resolve, Premiere Pro, and then I have a DaVinci Studio render, and then I have another,  So what I'm doing on this one is I'm trying to see if DaVinci Studio, DaVinci Resolve, Premiere Pro, and are actually fitting out the same, when I try to do the same settings, 2398, and then ProXQ, P3D65, is the metadata the same thing?  Is there a tag? There is a tag. I do remember after doing this, there was a tag that tells the player or tells something, hey, this was created in this, and all of that, and I also wanted to see if the bitrate would be different, because it's coming from the same file.  Ideally, the bitrate should be the same, the file size should be the same, but if you look over here, I don't know if you can see it, they're not the same.  So 6.7 gigabytes, this one says 86, 85, 81. 97. So, like every single software has even the file size.  Yeah, because I have memory of the compress differently. So, in this situation, it would be nice, and this is kind of what I would do, is I would grab all these files, and let's go back to MediaInfo, left, and I would drag all this in here.  It's a big file, so it might take a second.

54:40 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And it will show every file side by side? Side by side, yeah.

54:49 - Yukihiro Taira (Rustify.rs)
  And it also probably has to download it from the internet, because it's, oh yeah, let's not do this, let's kill this.

55:03 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And this media info, it's written in which language, do you know?

55:09 - Yukihiro Taira (Rustify.rs)
  I don't know. Let's see.

55:12 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I guess it's C++, I guess, or something like this? Maybe.

55:18 - Yukihiro Taira (Rustify.rs)
  It does crash a lot, so I'm not sure. Let's see, media info. Okay. Where is media info, and media info, and then show package contents, macOS.  Well, I'm guessing it's probably in Swift, to be honest, because it's macOS. But yeah, we'd have to jump in here, you know, and see.

55:50 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I guess if you do, I don't know if I should use that or not, because you have a crate that is called Toke in Rust.  Okay. And that... It gives you all the languages used inside any kind of application, and I think if you were using it, you would find the number of lines in the different languages.  really? How do I do that? Yeah, can check it in, yeah, tokay, rust, put rust after, yes, like this, and yes.  It's written in rust also, and if you scroll down, you will see what it looks like, yeah, it will give you something like this in the output.  Oh, okay, okay, okay, yeah, let me, oh, I love this. Yeah, that's basically, yeah. So you can check it with this, and you, yeah, let's see.

56:54 - Yukihiro Taira (Rustify.rs)
  Okay, I'm gonna stop sharing my screen, because I feel like I've been talking too much. Yeah. can see minutes, yeah.  Right.

57:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, but I understand better. So I think for next time, the idea would be to have more information when we drop the file and also, if possible, to have the rendering where you can go to, if possible, to the different frames.  I don't know if it's hard to do or not, but if we can, that would be good. And also, because at the moment it's diagnostic, but do you think it would be helpful to also, you were mentioning, you know, a process to do, maybe not streaming right away, but to do something from when all the diagnostics are good.  Do we think to do something after with this video file, or? It would be nice to be able to compress or to, you know, encode.

57:57 - Yukihiro Taira (Rustify.rs)
  But right now, I think... Doing something simple would be a lot more easier, I think, for now, and getting that down so that people will trust the output.  Yeah, of course. And then from there, if we have a tool that we know this is correct, and it's looking at the data correctly, then we can use that tool, the same tool inside, to start converting files and looking at it.  Because that will have a data point of like, okay, this is MP4, this is, I don't know, AVI, and we know that what we're looking at is correct, and then we take that file, that information that we have that is correct, and then we can go in between each other.  Because if we don't know what we're looking at is correct, we can't really convert it.

58:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, definitely. definitely. Also, I think I will also deploy the application on the... On On the... On the VPS, I already have to make sure SweetFox online and that it's accessible.  think it could be easy to do. So, yeah.

59:11 - Yukihiro Taira (Rustify.rs)
  Okay. I know it's already been an hour and I've been talking a little bit. I feel bad. But I want to talk about the course in general.  I've been doing the video. I've got to the point where I'm like, okay, need to actually be, I need to be coding every day.  So I'm trying to do at least a little bit every day and, you know, learning syntax and everything. But it seems like on day, on week three, it changes the curriculum a little bit.  Yes. Can you explain that I'd just be following the videos and just doing it that way? Okay. That's cool.  Okay.

59:55 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And you are already on week three then?

59:59 - Yukihiro Taira (Rustify.rs)
  Um, I... I have one more video on week two that I couldn't do yesterday, but I'm already, yeah, I'll be on week three, starting today I guess, I do have time today, but should I just be following the videos or anything?

1:00:16 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, it's more like, as I said last session, as long as you are sure that you understand the video and that you understand the pattern, that's the most important.  Also maybe, yeah, spending a little time on the looking at the exercises and understand also the exercises, maybe not all, but maybe the first one just to make sure you understand the pattern.  And the most important is more like on week three, because it's based on the project, and this is a little more important.  So to spend a little more time on the different project on week three, this is important. So this way you will understand.  It's on not only the patterns when we code, but also the architecture and all that. And on the week 3, you will find there is, I think it's the last day of week 3, it's focused on the authentication.  So maybe this part, you can just look at the video, because actually, when we do with Dioxus, so you remember it's a full stack framework, so it's not only backend, so it's a little different, so you will see the way we understand the authentication is different with Dioxus.  And we will spend more time on Dioxus, so it's not necessary to spend too much time for the backend specifically.  But yeah, after, yeah, I think all the others are important, so yeah.

1:01:53 - Yukihiro Taira (Rustify.rs)
  So I think my question is, so how much time should I be spending on Dioxus? You know, dioxid, like, Protege QL, Protege SQL, like, stuff like that.  Should I be studying? Because I did spend the first few days of last week with the Protege SQL. I was, like, looking at it.  I mean, it's not super, you know, hard. It's kind of difficult. But I did spend a good chunk of time because I didn't have it installed, and then I got curious.  And I was, like, okay, let me look at a few hours of, like, video. Like, I spent, like, at least five, six hours just doing that.  Okay. But then I'm, like, okay, I haven't done any of the other videos. Like, so I guess I'm trying to find where should I be allocating most of my time right now?  Should I be focusing on, you know, the syntax and the basic part of Rust? Or should I be looking at dioxis, you know?  And I did build, you know, like, I did follow, like, a couple of the tutorial. You know how it's  The tutorials, you know, it's just like, oh, let's make the dog thing and hot dog or whatever. You know, I did that and everything.  And then I was like, okay, this is basically CSS, you know, a lot of it is CSS and which makes sense.  And I was like, so should I be focusing on that part of it or like what we're like, I just need some guidance of like, because I only have like, it's not like I can sit here all day because I have other stuff I need to do.  And I just need some guidance of like, where should I be allocating my time more than anything? Yes.

1:03:37 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So the importance on the week three, as long as you understand the videos for week three, that's the most important.  If you can dedicate a little more time on the project, that's also good, but just to understand because every time you have all the solution also for the different projects.  So maybe if you want to speed up a little bit, you can look better. Thank you. to understand the different patterns that are used, maybe use the LLNs to understand a few things if you want, but you can go quite fast on this, and when you have done week three, the week four is on the front end with Leptos, so it's with Leptos and Net Deoxys, so you can go quite fast on this, just to understand the different patterns for Leptos, it's still interesting to understand, but you don't need to do the exercises and so on, just to just to see quickly the different patterns.  Same way or so for week five, it's for Stack, it's still using Leptos, so you can go quite fast on this, and yeah, maybe spend, with Deluxys actually, we are doing the project in Deluxys, so you will pick up things as long as we do, so whenever it's possible for you, because you will we can  The basics, quite fast, is to focus on async and multithreading. This is the most important to focus your time on, for the technical interviews for ROST.  So if you want... Say it again, what is the most important thing? Async, asynchronous. Async?

1:05:22 - Yukihiro Taira (Rustify.rs)
  Async, yeah.

1:05:25 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And multithreading.

1:05:27 - Yukihiro Taira (Rustify.rs)
  What is Yes.

1:05:30 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Multithreading. Oh, multithreading, yeah, yeah, Sorry for my accent.

1:05:36 - Yukihiro Taira (Rustify.rs)
  yeah.

1:05:36 - Maxime Montfort (maximemontfort.pro@gmail.com)
  No, So yes, asynchronous and multithreading. So this is the most important. I will give you some resources if you want.  But you can go quite fast, actually. As long as you can just watch the video very quickly, the solutions, and you can go fast.  Yeah.

1:05:58 - Yukihiro Taira (Rustify.rs)
  Okay. Okay, cool. Alright, so I think what I'm doing already is kind of like that, because I'm not doing all of the stuff, I do a few, and then I'm like, okay, I get it, you know, because if I do like a few, like syntax problems, and I resolve it as like, oh, a stroke needs to be like this, and then I do another one, and I'm like, okay, I got this, and then I go to the next video, so that's how I've been doing it, but I just don't know for the project, the FFMVAC project right now, I do have ideas or anything, but I'm just not, I don't know what's going on, to be honest, I'm still like, I'm looking at it, was like, okay, so I don't know, what is what, like, where is this, you know, and maybe that's okay, but I wasn't sure how much I should be like trying to focus on.

1:06:45 - Maxime Montfort (maximemontfort.pro@gmail.com)
  This is idea, as soon as possible, to focus attention on your time on the project, yes, that we are doing, this way you will understand better, but full stack and so on, and  Yeah, so to finish the week three quite fast with the last video, and then week four, week five, you can go very quickly on this just to check the different patterns that are for Leplos, and go fast on this, and then start right away to really start the project with Dioxus that we are doing.  Also to put maybe some everything, because you tell me that it's normal if you don't get it all that we are doing the project.  So you can ask also elements to better understand also. It helps a lot. Not to code for you, but to really understand the pattern and to ask questions.  It's very helpful to understand better. And also like to have a critical view on this. If you think there are some patterns or some things that you would like to include also, you can either ask me or so, or...  Or what you want, but yeah, so yeah, so quickly on week three, and then week four, week five, videos quickly, but go on the Deoxys project as soon as you can.  And then from this, after, there will be asynchronous AMBTS reading, but we can see that in one week or two weeks.

1:08:19 - Yukihiro Taira (Rustify.rs)
  Okay, okay. All right, I guess I'll trust the process and everything. Yeah, I'm a little bit like, okay, I'm never gonna, I'm not, I'm not doing anything.  I'm just watching tutorials, but I have been learning more and more. It's, it's, it's crazy because it's so fast paced that it seems like it's been like two months or something, but it's only been like two weeks.  There is a lot of things.

1:08:48 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, definitely.

1:08:50 - Yukihiro Taira (Rustify.rs)
  But, okay, cool.

1:08:51 - Maxime Montfort (maximemontfort.pro@gmail.com)
  When you, when you, when when you, when will be focusing on the project, you will see, you will be more proactive also.  That I think that guess that will be be in.

1:09:03 - Yukihiro Taira (Rustify.rs)
  I believe that. I definitely believe that once I have something to focus on, I'll have more of a hands-on experience, and every mistake that I make will affect me even deeper, and it matters.

1:09:18 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, because it will be your project also, so you always learn better.

1:09:24 - Yukihiro Taira (Rustify.rs)
  Okay, cool. Cool. Cool.

1:09:28 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So, yeah, thanks for our time, Yukihiro, and we will keep in touch, and if you have any questions or so on WhatsApp or Discord, you can ask me.  Cool. Thank you so much.

1:09:39 - Yukihiro Taira (Rustify.rs)
  And, yeah, like always, have have a good day. Bye. Yeah, bye.
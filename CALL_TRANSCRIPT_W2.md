Yukihiro <> Max - Rustify - June 15
VIEW RECORDING - 76 mins (No highlights): https://fathom.video/share/uBiWxFZyoM4qoxMwxzrsHcbZWscJNw7-

---

0:00 - Yukihiro Taira (Rustify.rs)
  Sorry, I'm disoriented, I'm sorry. No, no worries.

0:06 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Normally I put the same aware than last week, but we can change if you want to choose another aware that's good also.

0:16 - Yukihiro Taira (Rustify.rs)
  Oh no, this is actually perfect. I've just been extremely sick, and I was trying to catch up, excuse me, I was trying to catch up, and I kind of lost track of time, to be honest.

0:33 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, and do you feel a little better now, or are you still sick?

0:39 - Yukihiro Taira (Rustify.rs)
  I feel better. I've been sick for like three weeks, and it's been, yeah, it's been difficult. Okay, so I hope you will recover soon.

0:51 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, hopefully, yeah.

0:53 - Yukihiro Taira (Rustify.rs)
  I mean, I'm starting to cough up stuff, so hopefully that means it's the end of it. Yes, okay, pretty.

1:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so given that you are a little sick, so maybe you don't have too much time to, so you told me I think on WhatsApp that you started to do the LinkedIn and so on.

1:20 - Yukihiro Taira (Rustify.rs)
  Sorry? To do the...

1:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, so yeah, think I remember on WhatsApp you told me that you started to...

1:29 - Yukihiro Taira (Rustify.rs)
  Do the LinkedIn? Yes, LinkedIn.

1:33 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, I just updated a little bit of LinkedIn. Okay. That's what I was doing right now.

1:40 - Yukihiro Taira (Rustify.rs)
  But it's still very bare bones. I took a picture. I'm sorry, I'm like coughing so much. So it looks a little bit updated, but there's a lot that I need to do still.  Okay, okay, it's not... Yeah.

2:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  The idea is more like to do this during the coming weeks, because I was not expecting you to do everything the first week, so if you didn't do it, no, no worries.

2:13 - Yukihiro Taira (Rustify.rs)
  Yeah, sorry. Sorry, I know I'm coughing a lot. Yeah, no, no. nice.

2:20 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So maybe today it's up to you. So either we can focus a little bit on LinkedIn if you have any questions, or if you want, we can also focus more on the application.  So I've made a simple demo to make sure that it was working, that it's working, and then that we have a good basis to start with, and then that with these good foundations, it would be easier to make the application during the coming weeks.

2:56 - Yukihiro Taira (Rustify.rs)
  Okay.

2:57 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So, yeah, so it's Do you want to focus a little bit on LinkedIn, or do you want to start right away with the application?

3:10 - Yukihiro Taira (Rustify.rs)
  I'll start where you think we should start. I'm fine with anything, really.

3:15 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, sure, sure. So yeah, maybe I will share with further applications then, because there is a lot of things also on this, so I will share my screen.  Do you want me to zoom a little, or I can zoom a little if you want, zoom in, so this is you will see a little bit.

3:43 - Yukihiro Taira (Rustify.rs)
  I mean, I can kind of see it. I wonder if I could do full screen, if I do full screen with the pull.  Okay, and if I zoom out like this, do you see properly?

3:58 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Oh, yeah. Yeah, I think it's kind of, it's very, but yeah, actually. So yeah, just tell me, I will first show you the, on the, so this, remove, on the, on the application running, it would be a little easier.  Is it, yes, it's not, boom, I just need to restart, take serve. So yeah, so I'm, I use Dioxys for this, so I think I've told you a little bit about, about this last time, but with Rust, you have different full-stack frameworks that you can use, you have either Leptos or Dioxys, and Dioxys, it was made at the beginning to be cross-platform, so it's perfect for you, for your use case.  So this is why we, we use Dioxys for this, and as you remember from last time, you, you remember that it was kind of similar as React, actually, so, so it will.  It would be kind of easy to get started with it, so I will just wait that it compares all, and then I will show you the demo how it works, I will show you the architecture also, the important pattern, and then we will see together what features we can add in the application.

5:29 - Yukihiro Taira (Rustify.rs)
  Does this build on the server, and then once it's built on the server, it just works? Or does the client or the user need to build it every single time?

5:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, good question. So for this, so because I'm still quite a little new to Deoxys, I was mainly using Leptos before.  For Deoxys, you see, maybe you were asking because you have season, maybe. So yes, it's running. So I know that with Deluxis, I think it's SSR mode also.  So it means that it runs on the server first, and then we have an iteration on the client for all that is reactive.  So it runs on the server first, and then I did a few tests. So maybe I will show you that first.  So I'll just do this. Okay, so I've made it so that when we drop a video file, we get some data from this.  So for instance, I'll just drop this one inside here, for instance. Okay, and so you see we get some metadata for this.  We get also some thumbnails, also automatically. And then we have the possibility for instance. I just created this to make sure it was working, and under the hood, this uses the server functions, so it means that all the computation for analyzing the video and then compressing the video and so on, it is done on the server.  Yes, not on the client, on the server. So we get this, and we get also this with FFProb, so it's not FFMP, it's FFProb that I use for getting the data on Json.  And it seems to work fine, so I never worked with video before, so I was just testing, but it seems to work fine at least to get first the data.  that we need. It works also for the compression. For the transcode, I don't remind exactly if I tested or not, but at least the basic features work.  Maybe you have some questions about this?

8:16 - Yukihiro Taira (Rustify.rs)
  Question, is there a way to use your camera? Oh, yes. Like, if you could plug in, because this is a file, if there's a way to plug in like a live feed to like your camera, like if you're capturing something on a capture card?

8:38 - Maxime Montfort (maximemontfort.pro@gmail.com)
  With Dioxys, I don't know. I don't know because, yeah, I will have to check this. Yes, good question. I will have to check this.  Because the main feature that you would like is to work with streaming, you mean? With a live video?

8:58 - Yukihiro Taira (Rustify.rs)
  Mm-hmm. okay okay okay well um not not necessarily i'm just asking um because this is a very static you know and this is fine you know it's more than enough but um and it's probably useful for a lot of people if you can do it online but if you could if there's a way to like do the camera um and have camera input to go in and then it can either transcode it or um how can i say like convert that into something usable but i mean that doesn't that's i mean i'm thinking about something yeah and also yes also what you can do because uh there is uh you know like the the code here so what you can do that that would be very helpful so either you you can uh create a new md file for instance with all the the questions like this that you have because it's very uh

10:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Very good to have this. Or if you want, you can also on... So I don't remember what it is.  So it was this. So yes, you already have used GitHub. Yes, I think so. Yeah, yeah.

10:16 - Yukihiro Taira (Rustify.rs)
  Yeah, okay, okay. Okay, perfect.

10:18 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So you know you can create some issues. So you can also create some issues. For the moment, I've just created a repo on my own because I think you didn't create the GitHub repo yet.  But if you can create your own GitHub repository, this way I push the code and it will be on your GitHub.  It will be a little easier.

10:41 - Yukihiro Taira (Rustify.rs)
  Um, wait, what do you mean? I did create the one that said FFmpeg, but you want me to create another one?  Ah, no, no. This one is good.

10:50 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Sorry, man. But okay, so I will push on this one then. Okay, okay. Okay.

10:54 - Yukihiro Taira (Rustify.rs)
  Yes, yes.

10:55 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I will push on this one. It's just that it will be not... Exactly, FFmpeg, in the sense that it will be more like GStreamer or something like this, yes, yes, yes, yes, to something more, yeah, GStreamer or something like this, or video app, or as you want, and then I will change, and then let's see this one.

11:31 - Yukihiro Taira (Rustify.rs)
  Now is it... start, oh, I changed the name here, settings, there we go. I'll do YStreamer.

11:45 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, so very good, and if you can show me the link again on the chat, I don't remember the link from the table, this way that if push right away, it would be easier.

12:00 - Yukihiro Taira (Rustify.rs)
  Let's see, where is it? How do I, how do I end? Oh, here we go. Oh, but then the repository, wait, hold on.  Even though I changed it, it still says FFmpeg. Why? Okay. Wait, let me change it again. It didn't change.  Okay. My underscore code and chat. There we go.

13:08 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So you created, yep? Okay, Okay, perfect. Okay. Okay, perfect. So I'll just take this. So I don't remember who it's doing.  Okay, so it's this. Okay, so we'll just reset the new one. Okay, take this. Okay, git remote B. Okay, so perfect.  Git push origin name. I will just force like this. Okay, so it will take a little time. This way, you will, and maybe you can create a new tool then, or maybe you can create on your site the issues that you told, you know, the live, plug-in the camera.  This way, we don't forget. I will have to check if we can do this with Dioxys, if that's a good idea.  Okay. So now, if we refresh normally, we have the repo. Okay, perfect. So we have this. So, yeah, so this was for testing that we get the data.  Do you see other things? Because I don't know, for instance, in the JSON that we get, what is actually useful or not for, I don't know, for the bitrate, for instance, the duration, or...

14:56 - Yukihiro Taira (Rustify.rs)
  Okay. Um... Uh... It really depends on what we're trying to do, but bitrate is definitely useful, frame data, so you were kind of there, it said frame.

15:12 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so maybe I will just create an issue also with this, I will just copy and paste what I get with the JSON here, and maybe if you can check what is actually useful or not useful.  So this way we can keep only the relevant data, maybe it will help also. So I will just create a new issue for this.  So keep relevant data for JSON. At the moment we have this. Okay. Okay. Do you see other things for being able to read the data from a video that we will drop?  Or maybe you were thinking of other things?

16:24 - Yukihiro Taira (Rustify.rs)
  I mean, so far, it's very, you know, bare bones. You upload a file. Something like where if we could send the file somewhere.  So right now this is you upload a file, and then you change it, and then you download the file.  I think that's what the loop is. But if this loop could be like, I can send it somewhere, or I can transmit it to somewhere.  So have like an API, or some type of transfer structure where you can transfer data. I don't know. So even like to OBS would be fine, or to transfer to platforms, so if you were able to put a key, like a key for Twitch, then you can send out data to Twitch and have that play on Twitch or have it play on YouTube.  Okay, I see.

17:27 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And it would be more like kind of streaming or something like this?

17:34 - Yukihiro Taira (Rustify.rs)
  Yeah, so the problem with streaming is that if you make a video in a UHD or whatever, and then you just stream that, then your computer has to do all the downscaling and everything, which is fine, not that difficult.  But if you had a structure where you upload something, and then the server on the server, it will downscale it for you, and then it will send it out in the proper bandwidth.  Yeah. Yeah. Yeah. Yeah. Because you can see on here, your bitrate was, like, pretty high. Yeah, that's the, so you want that, if you're going to stream that, you want it to be at 6,000.  Hmm, okay.

18:15 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so, yeah.

18:17 - Yukihiro Taira (Rustify.rs)
  So, definitely way too high for that. And another thing is, sorry, so it's QuickTime MLV, which on Windows, QuickTime MLV is not really optimized.  Okay. maybe, you know, like, what would be helpful if that was H.264, you know, and then it will convert the bitrate, change it to H.264, and then send it out to a platform.  Hmm. So, I was thinking, like, a lot of, like, Twitch streamers now, they do, like, excuse me, they do, like, reruns.  So, it's kind of, like, they're not streaming, actually, but they're just, like, sending to the- platform like old episodes or old streams that they did, and it would be cool if they could, you know, if they're going to do a YouTube video, they can stream it using the platform to Twitch or whatever before they upload it to YouTube, or they can even do it to YouTube.  So they stream it before by uploading it here and automatically will make it efficient for the platform to read it.  For YouTube, it's not that much of an issue, because YouTube, you can send it whatever and it will work.  For Twitch, they're very particular about what you send them. If you send them something way too much, then they're going to cut down your bandwidth.  Unless you're a partner, which most people aren't. So you have to like stay in the bounds of streaming. However, you know, I don't think it's for me, it's not that technical, but if you're starting out, and you want to do something like that, where you're  We're sending, like, what do you call it? Shorts? Because that's another thing. A lot of people are, like, making shorts and everything, and you just want to send those to the platform.  Maybe you could have, like, a scheduler that keeps on sending out the shorts. You can make it on your phone, can make it on Premiere, whatever it is, and then this software would automatically make it into, or conform it into what it needs to be, and then sends it out via the schedule.  Okay, okay, see.

20:34 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Like, for example, I don't know, for example, with your phone, to take a short with your camera on your phone, and then to use the software, for instance, to convert properly, to maybe down the bitrate if needed, or things like that, and then to be able to schedule and post this to the platform.  Yeah.

21:00 - Yukihiro Taira (Rustify.rs)
  That would be, I think, very useful for this type of thing, because, again, if you shoot it on your iPhone, then it's an iPhone format.  Yes, yes. You know, so that's another thing where, you know, a lot of, if you're on Mac, it doesn't matter, but if you're on Windows, you really have nothing to do.  Okay, interesting.

21:24 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, okay, okay. Yeah, good idea. So, okay. So, yeah, after the code, I think I will also create the issue for this, just to think about this.  Okay. Okay, very good. And yes, so after the stream and so on. Okay. I can show you, if you want, this for the workflow.  So I've created some simple examples just to make the components proper for the workflow. So this way, we can reuse some stuff.  So it works kind of fine, and I've just, so there is a different demo, I think it's also, I've pushed it, there exists on the website, so if you, so where is it, I don't remember where it is, workflows, so you can check also on this link, up here if you want, so you can check.  So it's still a work in progress, but I wanted to have something simple working, and then, so if go on the application, so what I do for now is just that I drag and drop, for example, a video, and then the idea would be to, yeah, to have some stuff working here, so you can see the workflow, what is being done, and so on.  I don't know how it works on Jstream, or Jstreamer, I don't remember the application, or maybe, maybe,

23:00 - Yukihiro Taira (Rustify.rs)
  Oh, basically, yeah. So it's the same thing. So you drop in your file like that, and then you have, say, like color grading, or you could have transcoding, could have resolution, compression, or whatever it is.  And then I like the subtitles with SRT. Like, if you could put it in the SRT file, that would be awesome.  You know, and then having an output, I don't, so the problem, is that output, is that really an output?  Because you have the SS, but what are you outputting it to? So that output also needs to have an output.

23:41 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. You know what I mean?

23:43 - Yukihiro Taira (Rustify.rs)
  what is the end goal?

23:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Sorry, I don't get, I don't, sorry, can you maybe a little, uh, explain to me more?

23:48 - Yukihiro Taira (Rustify.rs)
  So when you put the SRT file together, you need to use a format that can support SRT. Or, basically, it's called 702.  702 captions in North America. It's a little bit different all over the world, but as far as North America, it's 702 or 608.  Well, it's 608 or 708. Anyways, there's a caption format that you can embed into a file that all the devices in North America read.  So when you're watching TV or you're watching a video or something, and then you put on captions, it's reading that file.  So it has to be in a certain format. So adding an SRT is one level. So you add the SRT, and then you have to convert that into the 608 format or the 708 format.  Okay.

24:41 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And then, after that, you have to encapsulate into one foot.

24:45 - Yukihiro Taira (Rustify.rs)
  Then you have to lock it down because you're just layering at that point, but you're not closing the data.  Yeah.

24:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. I didn't get it this way. Okay. For me, because I don't...

24:59 - Yukihiro Taira (Rustify.rs)
  Yeah.

25:01 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I saw that when dropping, because in any videos, have both the video and also the audio, and I was thinking that it would extract the audio stuff and extract, you know, maybe the text, from this audio, and convert it to SRT, then to be able to change it, or things like that.

25:36 - Yukihiro Taira (Rustify.rs)
  Oh yeah, well, that would be something with the other one you were showing me that had Claude on it.  What is it called, that AI agent that does, what, Lydia? Sorry, I know I'm like coughing so much. There's like an AI agent, I can send it to you.  There's a podcast I listen to that always pushes it. So you extend that information. That audio file through that AI, and you would have it process it.  There's also free software out there that a lot of streamers use.

26:07 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I know that there is a Whisper, Whisper.

26:10 - Yukihiro Taira (Rustify.rs)
  Whisper, yeah, that's it. Yeah, Whisper, yeah. You would have FFMPic, right? So just going back to what you have on screen.  And then you would have Whisper, and then from Whisper to Subtitles. Yes. And then at the end of the chain, you have to have a final export.  So something that, after we did all the processing, because what FFMPic does, it opens up the file. So it says, let's open up the file, let's look at what we have.  But at the end, you do need to close the file. So there needs to be a close bit, and that's like, do you use Premiere or anything for video editing?

26:51 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Not that much. Yeah, not that much. How do you render your videos?

26:58 - Yukihiro Taira (Rustify.rs)
  Yeah, yeah. So... me, I'm using Screen Studio.

27:02 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I guess you know this one. Screen Studio is very good. It's very simple.

27:06 - Yukihiro Taira (Rustify.rs)
  Okay.

27:07 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It's just, it does almost everything. It's more like, yeah, you can record yourself, and then it will add a nice UI on top of this with a smooth animation with a mouse and everything.  But yeah, I don't use the, yeah, yeah.

27:28 - Yukihiro Taira (Rustify.rs)
  So what that software is doing without anybody knowing is at the end, it's closing the gap. So the file is open, and then it does everything it needs to do, and then it closes it.  And that's exporting of the file. And after that, I was talking about, so after we close the file, we got all the data, we changed everything we need to change.  After that, we need to send it out to either, let's send it to a QuickTime MOV, or we want to make it to a H264, or we want to send it  this to a platform like YouTube, or we want to, I don't know, do more processing, like, after that, so we close it up, we do the bare bones, and then what if I wanted to, which is unrealistic, you should do this before, but I want to change the color, I want to make it black and white or something, you can do that, I don't know why you would do it that way, you would probably do the black and white before you close the file, but, so basically, that's why I'm saying, this chain right here is incomplete, because it doesn't have the end goal, in line of what we're doing, yeah, okay, okay, yeah, okay, see, yeah, we open with FFmpeg, but it doesn't close everything, and then we don't until it exports to a platform or whatever, okay, yeah, even if it's exporting to your computer, there needs to be a node that can do that, yes, yes, mm-hmm, okay, okay, makes sense, yeah, okay, and on that,

29:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So the application you were showing me, was JStreamer, if I remember, JStreamer?-hmm. Okay, okay. And is there, I guess there are some, oh, yes, maybe this, I don't know, example, JStreamer pipeline.  So is it something like this? Yes.

29:22 - Yukihiro Taira (Rustify.rs)
  Okay. Yeah, so you have the DMOX, so read file, detect file type, so we have that already, you know, with metadata.  DMOX, so we're breaking it up open. Okay. And then it's queuing video buffers, audio buffers, you know, that's the bitrate part.  And it's decoding, both of them. And then the only thing that's happening right now is it's adjusting the audio volume.  So this is a audio. Okay, I see. Yes, okay, okay. And then it plays, it's playing for this. So this is the clothing part at the end.  So it's playing the audio and it's sending the audio and the video. Thank

30:02 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And somehow, guess that for closing it properly, they need to assemble again both the video and the audio?

30:12 - Yukihiro Taira (Rustify.rs)
  Not necessarily. So sometimes you want to send the audio separate. Sometimes you want to send the video and the audio separate to whatever it is.  So like, say you're in a studio environment, and you put in a file, and then you have an audio engineer.  So the audio engineer doesn't need the video. They just need the audio. So sometimes you would send that, break it out, and then you send that to the audio engineer, and then you send the video to the video engineer, and the audio engineer will look at the video coming out of the switcher, and then he would adjust the lag and adjust the sound.  Okay, I see. Okay, okay, okay, okay. Because this is used in a professional environment, so a lot of times it's not just one person.  don't anything And if you have multiple people at a very high level, in audio, video, whatever it is, so a lot for this workflow that you're showing me, it's probably they're going to spend it.  Oh, yeah, middle one right there says pipeline on the bottom. Is this that one? Yeah.

31:21 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so it has a sync.

31:27 - Yukihiro Taira (Rustify.rs)
  So the sync is what I was talking about. It's if you think about a sync and then you're pouring water into the sink and it goes into a drain.

31:36 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay. So the sync is where you're closing the file.

31:41 - Yukihiro Taira (Rustify.rs)
  You're putting it in a sink and it's going through a small drain. Oh, okay, I see.

31:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, okay, okay, makes sense. Yeah, this is also separate.

31:52 - Yukihiro Taira (Rustify.rs)
  So, and another thing to, the reason why they use the word sync is because video is kind of like a.  word. I All A water stream, you know, you can, it goes from A to B, it doesn't go from like 99% and then it's, it's like each frame has to be sent in conjunction.  So the prior frame needs to be the actual prior frame or the encoder will not understand data. So each frame needs to be in sync and you have to send it next, next, next, next, next.  With a traditional internet file, you can just send whatever is more efficient. So if you're just downloading something, you can just grab whatever, like, oh, that's a quick file.  Let me just grab that. Let me grab that. And then you can pile that at the end. But with video, it's really important that the frame that came before me is the actual frame that is before me.  So it's kind of like water. And then what you're doing with these types of tools is you're opening up the water stream.  You're making it and you're putting it into a sink and you're filling up the buffer. So that's true. true.  Thank you. So that it's kind of like backed up, and then at the end, it goes back into the stream, because it comes out as a stream, and then it hits the wall, and then it fills up and get backed up and lag a little bit, but at the end, it is a stream that comes out.  Oh, okay, I see. Okay, that makes sense.

33:16 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, I see. Okay, that makes much more sense for me. Okay, I understand better. Okay, okay. And maybe in the UI, because here, I guess it's more like a kind of schemas to explain the process, but I guess for the UI, it's different.  Yeah, it's something like this in the UI of GStreamer? Yeah, it's more like that, yeah.

33:50 - Yukihiro Taira (Rustify.rs)
  Okay. And it crashes a lot.

33:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, okay. And do you know a video I could check? So, or... Or maybe, I don't know, if you have a JStreamer on your machine so that you can maybe show me very quickly your process so I can see.

34:10 - Yukihiro Taira (Rustify.rs)
  I actually don't have JStream, but I do have TouchDesigner on here. So TouchDesigner is kind of similar, it's just in C++, but I would need to share my screen.  Oh yeah, absolutely. Okay, so it seems like I'm sharing one screen to you. Yes, I see it. Okay, let me put a TouchDesigner.  For the moment on the screen I see, okay, perfect.

34:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So you see this?

34:55 - Yukihiro Taira (Rustify.rs)
  Yes, yes. So let's say we have a... So there's... There's, in this software, you have comps, chop tops, which is video stuff, channel operators, so anything that has a channel, left, right, RGB, whatever, more data, focus, audio.  Top, this is going to be all your 3D data, anything that has more than three coordinates will be in here.  Material, this is a material, so when you're projecting light onto a surface, this is what that is. Data, this is where your coding is, so you have your for loops, your operator, excuse me, can code, web pockets, XML, but just for this software, you know, but for the sake of this, we're going to do a touch-in, so this is touch-in, and then, wait, no, that's localhost, not touch-in, what was it, I'm a little rusty on this, I used to be so good at this.  We didn't choose it, maybe, for a while. Um, where was it? It was camera. Cash. It says bin, but it was green.  Sorry. Picture tab. Substance of movie file. I mean, we could do movie file in. Let's do movie file in.  Let's do cancel. All right. So we a banana. And then what we're going to do is say we want to do a let's say we want to do a constant, um, to the color.  So we want to add, let's do like 50. Just change it here. And what do we So now that color has projected onto here by adding a constant, and then you want to do Touch Out, and when you click on Touch Out, you can do a network out, you can do it uncompressed, so this will now send it onto my network, or you can do, and you can change your resolution here.  So this is the part that I was talking about, so this is a Touch Out, because it's using the network, but there's also a file out, I think, movie file out.  So this is video codec, if we want H.264, VP9, Apple ProRes, all of that, and then you do the EXR settings, and then you can output this out.  Okay, so this will read it to, with the audio, so the audio chops. So where the audio is. So this is not only getting video, but if you had an audio coming here, like, let's see, audio device in, and let's just say you want it to be AirPods Max.  So as you can see, as I speak, the audio is coming up. And then we can add this into here by dragging this into here.  And now this video is, you can't see it on this software, but this video will have this audio that I'm speaking onto it while it's exporting.

38:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  But this is mainly, yeah, go Sorry, and here it means that it works kind of live streaming. Then you can define the things that you want for the audio or the video.  And then... This movie out, it will stream to another platform that you want, or something like this?

39:07 - Yukihiro Taira (Rustify.rs)
  Yeah, so you can do, so this is video design. So this is file out, but let's say we wanted to do a device out.  So if I have a driver device, like an external device, I can send this out. There's also a stream out.  So this is streaming out, so we can do RTMP sender, SRT, WebRTC, and you can put in the location, the connection and all that, and then it will stream into WebRTC.  Or you can do RTMP sender, destination URL, so this is like if you want to send it to YouTube or whatever, you put in the destination URL and it will stream it.  Okay.

39:56 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And this thing is...

39:58 - Yukihiro Taira (Rustify.rs)
  Sorry, go ahead.

40:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  I don't know, I was just wondering, so this tool is mainly used for live streaming?

40:09 - Yukihiro Taira (Rustify.rs)
  This is used for, a lot of the times it's used for either live shows, so like if you're a person that is doing a live show for like a band, that's what they use it for.  They also use it for media servers at like companies. Okay. So like, say you have a panel to show directions at the mall, you know, when you go into the mall and you have the touch screen thing, and when you press something, a video would show up type of thing, or when you go to the zoo, so you can use it for that too, because you can add buttons and code and send it.  It's also used in environments where you want to send a lot of video data simultaneously to different locations. Okay.  A lot of software can only send it to one endpoint. This software, as you can see, I have four endpoints right now.  I haven't configured them. That's why I have these errors, but I can send all four out at the same time from one file.  You can also download it for free. It's going to be a little bare bones because you have to buy the licenseware to get everything.  But if you just want to try it out, it's called Touch Designer. Yeah, also parameters, not parameters, where is it?  Sometimes you can go in to select inputs. There's a way you can go. I guess you can't go into these, but can you go into these?  If you can't block. Clone Immune Bypass.

42:03 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And like, for instance, the difference in the way you use this software, if you compare with JStreamer, for instance, what is different with JStreamer compared to this, for example?

42:20 - Yukihiro Taira (Rustify.rs)
  So one, JStreamer is a pipeline tool. This is a full-blown application. Two, this is coded in C++, and it supports Python.  JStreamer is in C, so there's a big difference when it comes to, like, how fast it is, and, like, in a production environment, you know, you don't want something like this a lot of times, it's too clunky, but JStreamer is, yeah, like, a pipeline tool, and also this is licensed.  So, if you want to use this, they do take money from you, but GStreamer is free, so it makes more sense, okay, so this is, this is, but however, with Touch of Zerner, because it's so visual, you can have instances where, so you have, this is like the web callbacks, so you can have code here, to support, you know, connection, connections, and all that, and this is what's different from a lot of software, is that it supports actual code.

43:36 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, that's cool, okay, so you can define the code that you want, so in Python or whatever, and it will execute that code?

43:45 - Yukihiro Taira (Rustify.rs)
  Yeah, depending on what it is, I think mostly it's going to be Python though, I'm not a big Python person, so I really don't know.

43:52 - Maxime Montfort (maximemontfort.pro@gmail.com)
  But I guess it's more like for, okay, straightening some stuff inside the pipeline or...

44:00 - Yukihiro Taira (Rustify.rs)
  Yeah. But yeah, so it has its quirks, and it has its good parts and everything. But basically, going back to where we're looking at, the idea is that you have a file, you change the file, and then you send it out to, so this is, there's no connection here, but the audio potential, let's see if I have an actual different file.  Let's see. I don't know if I have a different file. Movies. Resolved. What is this? No, I don't have a file.  It's the documents. Sorry, if I'm going too long, let me know. No, no.

44:50 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And if you, yeah, I don't know if you have, yeah, maybe a simple video, or maybe you can maybe download a very simple video to see the difference with.

45:01 - Yukihiro Taira (Rustify.rs)
  Yeah, okay, so this video, and I know you're recording, so I can't show you this, I can't legally show you this video, so don't share this part of the video if you share it to people.  No, no, no, no. Okay, so this is, this is a show that we did. Let me make a figure so you can see.  So this is for Amazon Prime. Okay. So this actually has, if you look at this video, this one, and so, let's see, does it say here?  So this has a 608, actually, while we do this, let me go to the actual file. Oh, where is it?  This was document, and then open with LATV TouchDivider. Do I not have that file? I guess I don't have that in this computer.  Fine. There's the software that I like to use, but we open the file, and we do Command I. So this file, as you can see, has BT-709 here, which is the color, so that's the color here.  It also has audio details, English mono, so it's not stereo. Each track is that. And then it has four tracks.  I guess it doesn't stay here. So there's four total tracks. So you have the visual track, and then you have the two audios, and you have a fourth track, which is the subtitles.  Okay. So this, if you were to put this through like a TV, it would show you the subtitles. Okay.  So this is a complete file. I can't really send you this file legally. Yeah, of Yeah, of course, of course.  But just to understand the process.

47:32 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And then, for example, so you have the video, and with this software or just streamers, the idea would be to take this video and to be able to stream this on different platforms.  Yeah, yeah, that's the idea.

47:49 - Yukihiro Taira (Rustify.rs)
  Let's see if I have something I can share. What about this one? I mean, I could share this. I mean, it is a company promo, but I mean.  Well, yeah. Well,

48:00 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe you can, yeah, maybe, but I understand better the workflow, so this is the most important. Okay, and use the idea for the application, it would be to be able to stream a particular video to different platforms, because originally I was thinking more like to have a tool to compress video or something like this, but okay, it would be more like in a streaming, yeah, in a streaming manner that you would like to use the application then, to have the application.  Okay, yeah, that would be ideal for me, I mean, honestly, we can start slow, and not everything has to be like.

49:00 - Yukihiro Taira (Rustify.rs)
  It's you know, efficient right now, but it would be ideal if, we can do that for this one, timecode, it's actually, yeah, it doesn't tell you enough data on, like, the Apple inspector, and that's the biggest problem.  Um, let's see, do I have anything else? Oh, there we go. So, let's see, I mean, I could send you this one.

49:35 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And, like, for example, when you, so when you start streaming on, uh, on different platforms, how do you, maybe it's a naive question, but, uh, how do you know that it's, uh, it's streaming properly, uh, with, uh, with a good, uh, all, all set up correctly?  Like, can you, can you have, uh, a feedback from the, from the software?

49:59 - Yukihiro Taira (Rustify.rs)
  Um, on. Let's see, let's go to, where is it? Are you still looking at my screen? Yes. Let's see, so on YouTube, sorry, where is it?  YouTube Studio.

50:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Ah yes, because you can do live with YouTube Studio. Okay. Yeah, and then you have analytics here.

50:31 - Yukihiro Taira (Rustify.rs)
  Okay, okay. And it will tell you your content, the bitrate, and all that. It will give you a live view.  But also, what you look at is, let's go to that GitHub you sent me. We have the, it was, where was it?  Issues?

50:53 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, it is this one. Okay.

50:57 - Yukihiro Taira (Rustify.rs)
  Yes. So, average print rate. It's for raw sample. You want to be looking at, does it have, wow, this is a lot of good data here.  There should be drop frames somewhere. Let's see if I can search.

51:28 - Maxime Montfort (maximemontfort.pro@gmail.com)
  you can, yeah, you can search for me. I was using ffcrop for this and not ffmpeg. I don't know if.

51:38 - Yukihiro Taira (Rustify.rs)
  So there's an idea called drop frames. Let's see, drop frames video.

51:55 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So this drop frame is.

51:58 - Yukihiro Taira (Rustify.rs)
  Yeah, so frame. Data, when it gets dropped, you add a drop frame, and then you want to make sure you have zero drop frames, and then also, if we go back to this, there's a time code, where is it, where do we put it, did I, Issues, time, time, creation time, time time, time base, interesting, so this doesn't give you time code.  Interesting.

52:54 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Maybe I didn't take all the data, don't remind, maybe it's possible to get this, I guess.

53:01 - Yukihiro Taira (Rustify.rs)
  I mean, a lot of times it doesn't give you timecode, but timecode is basically, if we go back to QuickTime, OpenRecent, let's just say this one.  So timecode is this right here. Okay.

53:21 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So as you play the video, see how that timecode goes up?

53:25 - Yukihiro Taira (Rustify.rs)
  So this timecode, if you look at it, it goes to 29, and then it resets. So timecode is not time.  Timecode is frame time. So this 40, I don't know if you can see properly.

53:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It's 19, 43, 17 right now.

53:47 - Yukihiro Taira (Rustify.rs)
  And as I go frame, fry, frame, and we hit 29, and then we get to 30, it goes to 44.  Okay. So because the frame rate is on here. 29.97, so it's a drop frame. Time code is based on the FPS.  So as the FPS goes up, and it hits 29.97, then the next one goes up.

54:15 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so the drop frame is, for example, you have a video, you can have either like 30 frames per minute or 60 frames per minute.  Yeah, 60 frames per second, yeah.

54:31 - Yukihiro Taira (Rustify.rs)
  Yeah, so when you're doing 60 frames per second, this will go up when this hits 60, and then you'll get another one.  This is time. So we're at 19 minutes and 39 seconds of this video. If you were to watch this, and you look at your clock, and it's 19 minutes, 39 seconds, this is the frame that you would get, right?  But in time code, it's 1944. Mm. This gets greater as the time goes by. this is at 45 minutes and 51 seconds, as far as time goes.  But now it's 45, 54, 11. Okay. Because it's frame data, time code is slightly different. So what you can do is you can compare, you can look at how this time is going up compared to the frame.  And if there's a difference in it, then you're slipping and then you'll get drop frames because then the codec is like, I did not get that framing.  So I'm just going to drop that and go to the next one. And that's when you're watching some video or you're watching, you know, stream and it jumps.  Yes. That's a drop frame. You can visualize it because you don't all of a sudden the encoder or the whatever it is says, I don't, I didn't get that.  So I need to move on. Yes, yes, okay.

56:01 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And this is why you were telling that it's important for frame by frame properly, like in sync. Yeah, in sync.

56:10 - Yukihiro Taira (Rustify.rs)
  So if everything is coming in mumbo jumbo, then you're not going to have a video because it's like, I need this frame right now at this moment.  I do not have it. So I'm just going to keep on going. And you'll have zero data. Yeah. So, and so, you know, like when you're, another thing is, so when you're playing this, I can like kind of analyze the things.  But say if, if I'm in software like this, so now it's playing, and I wanted to, let's do this, but you can easily see.  So let's say, let's go to touch out so you can see the end result. Let's MOV out. This gave me a preview.  Won't give me a preview. Let's add a null. That way it will give me a preview. Null. Okay, there we go.  So as you can see, before and after, because of this concept, I can keep on changing the color, kind of like this.  I can also add code, or I can add an oscillator to this, so that it goes like rainbow colors, you know?  You know, I can do that, and process it, you know, do some color correction or whatever it is. Other things I can do is, I can do like, what's a good one that is easy to understand?  A blur? A blur that's called The Black Bayou, actually. Let's add it to the null, so a blur right here, so you see it's blurred, and let's do switch, so see how I'm just adding it to here, and then I plug it into the null, right, so now I'm on index 0, because the top one is going to be this one, and then I just switch to index, the index, I can do a blend, like that, it's constant, let's make it something extreme, like white, oh not white, like pink, difficult because, let's see that, so it's very extreme, and then switch, and then now I'm blending through, okay, okay, okay, see how I'm doing that?  And I can keep on adding values here. I can also, you know, cap this parameter to different things. But say like, okay, I like this.  I like how it's like blurry and all decolored and everything. And this is the output we have. Just for looking at it's sake, I'm just going to keep it.

59:20 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, because when you put the null, it's because you want to have an output that you will be able to download after.  I don't understand because it's a null object.

59:37 - Yukihiro Taira (Rustify.rs)
  So right now, this null, this null is a, it's just, okay, we're going to compile it. This is a part where, so you can preview things.  A null is basically nothing. So if you just want to look at something, you put a null inside. So it's ideal to put a null before these things too, because  This can change, but the null will always be here, so ideally, if I was actually going to build this, I'll get a null, and I'll put this in here, and then from the null, I'll put this in.  That way, if I have a different video here, like say I have this banana video, I can just plug this into the null.  Okay, are you asking for the modifications to... Because I was on here, but I would put the flag on null right here, and then whatever's plugged into the null will show.  Let me switch that real quick. There we go. Well, before, after. Yeah, it kind of messed up there. Let me take it all off.  So we want to show the null first, and then we want to show the after, which I don't know why it's saying that.  But you get the idea, right? So the null is... just something where there's nothing there, it's just a midpoint.  So the biggest thing, the biggest thing is that if you look at what we did so far, you know, we add the null, we add the constant to change the color, we added the blur, we added the switch, none of this is gone.  So if I were to be in, we still have this audio device in my ear pods. So if I were to be in a software like this, which has layers, so right now it's only showing the top layer, the original layer was this, that was shown in my living room, and then I would have to make that, but because the original layer is shown, I would have to turn that off, and then do the final compile, right?  This is called a destructive workflow, because what gets exported or what is there is only the top layer, whatever you see on top is going to be exported.  All of this. Don't I have out. So if I were to export this out, load that file back into Photoshop, I only have this, what we see right now.  This, I have every single step. So let's just open up like a file that I made. Open, let's see, touch designer, projects, camera multiview.  Okay, this is going to be a big file. What? Oh. Is it going to come up? Yeah.

1:02:42 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Allow.

1:02:45 - Yukihiro Taira (Rustify.rs)
  No, no. Oh, there it is. It's on the other screen. Okay, so this is, let's see, I don't need this console, I don't need this console.  So perform, I got a perform project. Oh, is this just a snippet? Oh, this is just a snippet. This is the wrong one, because there's nothing in perform.  Sorry, open, camera multi-view, under, what's this one? So here we have here, so this is... is a multi-view that I made.  It's basically this marquee right here. Let's go back. Yeah, it's just a marquee, so multi-view test. And inside this is actually code of, I think this is a JSON table.  No, it's an XML. Yeah, it's an XML, and it reads XML, merges it in, and then it converts it to a string.  And then I have a null here to preview it. And then I've put it into a text node that is inside this program.  And then I convert the text to center it. I transform it so that it moves left to right with time.  I think this is time, yeah. abstime.seconds times 0.3. So 0.3 of time. I fit it into the screen, so I use a fit, and then I open it.  Okay. Okay. And then, so this marquee is showing right now. See how I have all of this information that I used when I made this a long time ago.  This is the benefit of this software is that when you open it up, everything that you've done, everything that you have shown is there.  So there's no point, you know, like there's nothing that gets lost. You know, like if I wanted to come back to this in two years, I will still have this.  Yeah.

1:05:30 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It's not, it's not destructive. You see, you still see the old workflow that you, that you were being using.  Yeah.

1:05:38 - Yukihiro Taira (Rustify.rs)
  So in a environment where you don't know what you're going to do, you know, like you, you just need to start building things.  Yes.

1:05:46 - Maxime Montfort (maximemontfort.pro@gmail.com)
  It's very powerful.

1:05:47 - Yukihiro Taira (Rustify.rs)
  When you know exactly what you're going to do, this is overkill. You don't need that, you know, but if you don't know what you're going to do and you need flexibility, this is just, you know, you, you can really just kind of like.  don't Do a lot of testing and all of that, yeah.

1:06:11 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, so let's say, for instance, you have a video where you want to test, I don't know, the luminosity or the colorimetry or things like that.  Then you can test it and make sure that you can go back in a few months and have all the sort of process, okay.

1:06:35 - Yukihiro Taira (Rustify.rs)
  Yeah, and you can also go back and change the code, you can have different variants. So that marquee was going at ABS time, so absolute time, meaning it's not timecode.  And it was doing absolute time and then .03. Say I wanted to go slower, can add, I can copy and paste that, another one, and make a slower version without destroying the other version.  Yeah, yeah, I can just slow Look at it. Okay, this looks good. And I can make a million of those.  All in different increments. I can even make a switch to switch to them, just so I can see which one I like.  And then when I have an idea, okay, I want this one, I can erase everything else or just leave it, you know?  I don't have to commit to one thing.

1:07:19 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, okay, very powerful. Yeah, you can just copy and paste the nodes that you have or put a switch and test, kind of habit test the things that you are doing.  Okay. Yeah. I guess it's possible to do this also in the application. Okay. But okay, so I understand better.  I will see how it's possible to do the streaming and so on. At the beginning, still, I think it would be easier to first make sure that in read mode, we can get all the data that we need.  Then in the right mode, simple things like compressing, transcoding, and so on, and then when this is working to include streaming, because this is more complex to do, but yeah, more static at the beginning.  This, I'm sure that it's easy to do for the streaming. I will have to check. And also for the, yeah, because with the access, so you can also have the mobile application.  And I don't know how it's done for the native features of mobile stuff. Like for instance, to record a video from your phone, and then maybe to apply some changes directly to the applications.  I would have to check this also.

1:08:46 - Yukihiro Taira (Rustify.rs)
  Yeah, I agree that in the beginning, you can just be a simple, you know, read the file, change things in it, and export, you know, I think that's more than enough.  And then the streaming part can come later. Mm-hmm. I mean, streaming is a new thing, so it's not like, you know, it's been around for a while.

1:09:05 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, and also what you can do on your side also is to check also the code that I've pushed on the GitHub.  This way you will get a little more accustomed to how it works with Dioxus. So yeah, we can do this.  Yeah, I'll pull that and build it in my environment.

1:09:30 - Yukihiro Taira (Rustify.rs)
  Yeah, first make sure it works and so on.

1:09:33 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, you will probably need to add a few things, especially for mobile.

1:09:41 - Yukihiro Taira (Rustify.rs)
  But you know, normally it should be good.

1:09:44 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Okay, and so by the next week, Yeah, to first make sure that read and write with static files work fine.  And then we will see after this. Yeah, okay. Okay. Okay. Okay. Okay. Okay. Okay, so we can do this.  Yeah, yeah.

1:10:03 - Yukihiro Taira (Rustify.rs)
  And then as far as the Rustify platform thing, I want to talk a little bit about that. So I did watch all the videos.  The issue I was having is you watch the video and it was like, yeah, I get this. I know about iterators.  And then I go to the thing and I'm like, wow, this is... There's a lot. There's so much stuff that I haven't learned yet that I need to learn.  And then I'm relying on Claude to help me. And I just felt like I was cheating, you know?

1:10:43 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, I get the point. Yeah, because it's... You will see the week one and week two, it's really based on exercises with a lot of exercises and so on.  And then from the week three, it's based on... So it's a different way of seeing this, but yeah, it's because what you are seeing that maybe there is a little too much exercises or is it...  it's just the learning curve went from here to like there.

1:11:17 - Yukihiro Taira (Rustify.rs)
  just went all the way up, you know. Yeah, okay, I see.

1:11:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yes, so maybe I think I will... because you don't need to see everything from the week one and week two, because we will focus more attention on Dioxus, and then the more you will work with Dioxus, the more stuff like iterators and everything will make sense also.  But as long as you understand the videos, that's the most important. And I know that after when you think you understand and then when you have to do it by yourself, it's more difficult.  But yeah, as long as you understand, that's the most important, because anyway, like in the future, we will all be using AI to code, we will not code by ourselves, by hand, so the system...

1:12:17 - Yukihiro Taira (Rustify.rs)
  I mean, ultimately, I enjoyed it. I liked the challenge and everything. I did learn a lot by doing it.  I'm just, you know, I was surprised because it started out very easy and then I was like, oh, with the ownership and the strings.  I was like, wow.

1:12:31 - Maxime Montfort (maximemontfort.pro@gmail.com)
  This is the most important, yeah. This is the most important to understand ownership, borrowing, to understand why we have this, where, who owns the data.  This is the most important. When you see a ROS program, you need to think who has the data, why is it moving, and stuff like this.  This is the most important. And the more you practice, the more it will feel natural. But But as well.  As as you don't understand this, as long as you understand also why we have the borrowing, how it works, that's the most important.  Yeah.

1:13:09 - Yukihiro Taira (Rustify.rs)
  So am I going to get five more this week to do? Yeah, sorry. Go ahead.

1:13:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  No, no, I just, yeah, because I think at the moment you just have access to week one. think when I did, yeah, I need to also give you access to other weeks also.  It's just that, you know, like, you have done the deposit, so this is why I did you only the week one, and then I was, you know, for the first payments, and then I can give you access to When, when do you need that?  Uh, in the, in the coming days, if that's possible for you. Okay. way, yeah, this way it's easier, and then I give you access to the, to the.

1:14:02 - Yukihiro Taira (Rustify.rs)
  Okay. I mean, I can give it to you now. Yes, yes, perfect. You $1,200.

1:14:09 - Maxime Montfort (maximemontfort.pro@gmail.com)
  Yeah, perfect. Yes, so I sent you the link just after the call. this way, just tell me when you have done the payment and then I'll make the other things.  Okay, perfect. Do you have other questions related to the...

1:14:28 - Yukihiro Taira (Rustify.rs)
  No, I'm still kind of stick and I'm still catching up. But yeah, overall, it's been positive, so... Okay, good, good.  Yeah.

1:14:37 - Maxime Montfort (maximemontfort.pro@gmail.com)
  So yeah, the most important, understand ownership, borrowing. This is the most important. Also week two, I will tell you after week two, you also don't need to do everything in week two.  So, but I will tell you that after.

1:14:52 - Yukihiro Taira (Rustify.rs)
  Okay. Okay, cool, cool.

1:14:54 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And yeah, if you have any questions, also your WhatsApp or Discord, so feel free to ask me. Whenever you have a question, so if there is something that you don't understand, or you think maybe an exercise is too complex, or maybe to just 10 years old on WhatsApp.  Okay, cool.

1:15:16 - Yukihiro Taira (Rustify.rs)
  Thank you so much. And thank you for your time. know we went over pretty long. It's a pleasure.

1:15:22 - Maxime Montfort (maximemontfort.pro@gmail.com)
  And see you next time. I'll send you the link just after.

1:15:25 - Yukihiro Taira (Rustify.rs)
  Okay, cool. Thank you. See you soon. Bye.
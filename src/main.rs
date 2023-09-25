mod quell;
mod render;

use std::env;
use std::path::Path;
use std::time::Instant;

use render::render::render;
use quell::codes::import;

use serenity::async_trait;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::command::CommandOptionType;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::ApplicationCommand(command) = interaction {
			println!("Received command interaction: {:#?}", command);

			if command.data.name.as_str() == "preview" {
				let mut level = command.data.options
					.get(0)
					.expect("Expected level option")
					.value
					.as_ref()
					.expect("Idk1")
					.as_str()
					.expect("Idk2");

				let ticks: usize = command.data.options
					.get(1)
					.expect("Expected ticks option")
					.value
					.as_ref()
					.expect("Idk1")
					.as_u64()
					.expect("Idk2")
					.try_into()
					.unwrap();
				
				let now = Instant::now();

				if level == "ao" {
					level = "V3;1q;1q;2{{{g{)0g4g{{4)a7)la2{{{G)d9)94)Vdu{{{8)-b{e)b9YG{2)Z4)0f6(1h)c2c)a7SS)l9g)f5)44i)g6k{4)f6{2{4o)s8{{{{Y8i{c4Y{{aa{6a666c)l4o{{2e{)14{{e{o{22cccc{cG(2P)64{{!(2Z)82(2^)6G{{0{{6g4(3h)6{{{6eAe(3F)8Y{g{a{g(3D)9S)9dY{{S)!5(2r)7aa(4m)9ii(2l)6GG{2(2x)a6{{G{{2(1v)6)G6aaaa!Aa)74(2P)e)u5(4G)64{c{g(1p)6a{a)?44{4(1$)9Y2o{c{cG(4D)78{4(58)bG)^68{{{!(1p)f{{{a(6T)cgY(2P)6a)m4)i4)f4{a{2{2{66(3%)6g)k4g{S{{c(3b)7(1m)7(7k)7(2P)6{{{{6a{{6{6ssss)w6(8f)ai{a{(41)6)36)N8c{{c)15{!0aa8g{{M(8e)c{{6Yg{2(1p)b(9c)98(7E)7(7U)9{{Yaa{8{{o)W6aa{)O4(3?)6Y(9&)7ag(8.)cc4)G42{i{{4(1p)b8)04)y7ea(52)9)99S{4{4(a+)7aa(1g)72(2P)6Y{ga{!A!(6P)622{o(1A)66a)0eo(2V)b(2R)b{a{cG0G(3I)6{{{4(97)8(c})fi)f7(4V)6(3x)6!888!(5t)b4c{{{2(6m)6(4e)84{2{{a{{88a(74)7)&42(3P)8G{{k(5t)7)u5c)04o{{!o(94)744{2)&5(5t)9(du)a4i2{{{0{Ae0e{aY)&6)B4s{{c)28(dx)8)X90YoYo)I4!)i7g{6{G(87)b(7M)6(5t)6eA{0(2P)d(1p)8gS{{6c)27G{{2)74(2P)62cY(2P)6(1k)6(3M)6o)36a{agg)Z6{{(6U)64i4{2i2(7F)7{a{{{g8a)04{s(3A)6)28(5j)a{4(7V)6(e-)8(6k)7Y{(3C)6Y)Z5a(iA)84{{e(ce)782{6cca)R4i{{a(5&)6(2Z)6(5G)6o)540)T60{g(1q)6{{44(2P)cc2a(1E)64c(9z)a4S4{2S(fO)6c24)S58(dY)9(1p)9(5w)7(8l)7g{Y0(e1)7{{)T4(7X)7Y{{4G(1E)7(ig)7(3&)b(3+)6{KY(9&)7!A!)k4a)27(7{)7)0b0)U6(9%)8Y(87)9S{8{24{{c%A%(5t)ga{{{gi(4O)7(aW)7{2)28(1o)b)a60(h2)aG(53)6(6T)6!G{0Y)C5%A%(nD)90{A)L9a(b3)7S{2{g)Z5)26(2P)82{4{(5d)60)C6(bP)b(43)6!G{0YYY(33)6)A56{{{4S(49)74{2{8g8(5S)7A(7c)6)26c{c{{oG)l6{{o(m-)8(jT)8c(6%)622G{{{{ccY(gP)9{(e=)ca{{6{G(oh)7{{{A)sc{c(58)a)le(rR)e(dn)8(9G)6(sc)b(fo)6a{(3$)7)R8(1p)6(gh)6c{c{a(s+)c(1C)c2{6!8g(2P)6Y(4x)6(1p)6(kq)b(i-)6)g4(1n)8GGG(3T)7(2t)b(58)6(u5)e)08(e.)6(5E)c(1p)9(2U)b(c=)7{(5t)6(57)6(iT)a)94K)Aa)qaY)+6!{Y(aJ)6Y(bM)7aaa{{a)C6G(1u)g(jZ)82(8y)6(3q)6A)c4E(3!)a(5E)h!(3-)84{!G{{g%%%%{6ac{S(6a)f6G{6aaG)748(8H)b{(7^)6E(ma)c(1p)i(45)b2(d4)62{6ccG{{8{a8a8(5F)64(3z)7)44)34i{{0{aY{Ya(9J)6a0siK%asS)44)9dg{!{o{{{Y(zy)9%(9$)6c2)^4g(wC)a{YYY(eW)90{{{go888gog888(jV)7(3i)8666eee(As)a(2P)9(wZ)8a)bbS(A=)b08(sD)6(2&)9{{oooA{0ooo)n8(vY)9(A+)d(1p)8Y(ha)6{{{%(6I)72{)Q4(4Z)96i{gYS(1p)8(72)j4)A48c(67)7c{688G{U{U(6T)a)X5!)76(CR)ac{e(3m)90{{8(1p)8i{{{g(n-)92(d2)74{iS(v9)8(7E)e(2P)7(iA)6(5t)8{{2g{GGSG)08)9a%2{)q4(fm)6o(b!)72{c{{4o{8!8!2c{{6!(gT)68YU+{{a(6T)6(mq)9(fe)94Y)U4(2P)e)r4)J4(2h)6(Gk)b(5l)94{!)v5(9h)8)74a(87)9(z2)c(dK)9(hM)7a8a(aE)6c)d4(1n)6(57)7(3S)a)97(1j)6)56W)74a{a(1D)7a6A68c(Ep)8)L4(1p)8o(4D)60{{c{62{o{2{6)Q4(7!)8(jd)a(1p)hG{(4b)6!4)H4(8k)6(3v)8g{GGG(fl)7o{{(2P)9(43)8(3&)e(l1)6)e8Y(1b)bW{W(li)b(4n)8)F7(np)a(18)9(eI)6Y{o(Me)b(5v)a6o{o(zn)6(25)a)04+(h!)aY(1p)6g(1p)7)X6)o4)H7c{{0(h1)7o%)046A6(jO)g(2d)7)&5(j0)7(h5)e!8)O44{c(5{)7(4X)7g{A(rA)7{0(1B)8A)74%)l6(MU)9)0r)t86!c!{{{I(rK)7(2P)a(e?)7(dI)94(b^)8(1%)6(4x)9(4d)7000(dD)8{2)$666Y(57)6(2P)6oo(2P)g(8.)8(JQ)8(w9)i(uE)6(2T)c(cQ)8oo6)06YYY{{{g6Ai(4S)6(5t)e)U5G(7Z)b(CV)6K0KK)h6(3^)d(29)6(bE)a(1O)6o4ccYY{{g(66)8c(bI)8(2P)9)89)N4(1o)6(iE)g)FeY{4(it)6oG(ju)6{c{YY{{4(4C)74(87)h4{{q(26)9(1o)8c2{62(c4)6(2Q)8(aW)9{Y{o)w4cco(gP)7(1p)7)=88(5t)fg)44q{S{{4{6{{4(2P)9%(R$)6cc(oK)7(IN)7g{e{Y{{2oc{oo{(1q)6(1p)9(2t)66o(87)f%(1p)6(cp)a(1o)6A)g566{oo)94A{{4cc2)85eY(oe)boG{{{occ4(1p)8(1d)6(2P)7Y(3D)6%(Ff)7(6u)a(5r)jo{o{{o6G{g{{{e(sL)7(H5)7G{{{ooGGY{{{2cccY(3m)7)u64(rd)9(Bj)8Y!A!(8h)7(5t)6($f)c(IF)8e(D0)d(46)7(8s)aYA{4{c(aE)7)06K(1y)6(RJ)7g(12)6)u4(k%)8(cz)9(9d)6(5t)6(eB)6{44{2{oo2{2(Qp)aY{{I(bM)f6%%%{{{S(C-)d4(E0)7ee(%s)b)x5A(2P)6(gE)96c{oo{{Go(JV)9Y)F5c(2j)6(NI)6%(3{)6So{{q0q(5^)6)45g{{iG{{{i{4{o(kF)8g{{{e{e{!{{4{6cY2{o{oo{Y{(1p)72)Y56c2A24c{CE)0444(5t)dg(av)7(y})8ee(5t)7A)?5{{o0(1p)9{86(1L)6(1p)c(d6)820(hc)7(x=)a(p.)e4(5X)a6(bt)7A(hO)b(.c)662(%t)6(1p)gA{{{4c2S(1p)768GA(1p)d4(&G)9(Jb)6{{G{{a(9b)9(lr)h(2l)6(1p)bA(4C)68c2A(1p)7068G(Pk)7{g{2{{2a(JD)io(cQ)74{o(Eu)c(eJ)7)Q46(1p)a(4X)6o(6T)9(AL)a(2i)6cg{2(3^)72)j4($0)6(J8)b(43)9Ye(%I)9(p})6c(1p)8(oQ)9{4(7S)7g4444msssAsc{{aq(5A)d42(bX)d{{g(wR)72(7b)9(mX)7o(hW)6(Y=)6Y{)?4!(?K)7(2b)62i2{s(&j)6qa)04(1f)7!($a)6Yao(?r)7(5K)7e(l%)b(6o)6(xM)6o{Y{{Y!YYYq(cB)62(Km)6aa)$5Ss(4c)6(2P)cM(6A)h{{o{{g(112)c{G{4(1o)6(2P)9c{Y{c{{I(1p)66(2R)a(Rm)8)07(eQ)8(X})6(2J)6GY(sy)c8)O6uuuu6o{{GGGY6(4h)6(43)66o(43)8(}T)6(qs)9{{(2P)cS(ay)6(13Y)9(CE)c)i76)04o{{Y2uo2(dt)6(5t)9(mM)7(3f)72)!5Y2oo)s6(.M)76i)49gg{{!!A!c(3?)7(4t)76o)04{{{6(2T)84(87)d(tK)h66{{{q{42(6.)6Y{M(2P)9(7$)9c(1i)62)O8ou(Eb)i!)?5{48(x5)gc{c6c(1p)6a4{s(AZ)c(gr)c(ni)f6ou(XX)6(Eb)84(1p)a8{(16x)b(1p)f224{(8k)6(1t)b(1p)9G(O1)eou{o{6{{{G{GY{6)a5(43)7(F%)8(p+)84(1p)b)04(1p)6EI(4n)6{g{YY(5m)9(12V)7G(s0)8($n)8624)I4g{{ggg)E4q{4Y{G{{Y(1})8(1E)8q{gKY{s(9V)6)s6S{{Yc++(UL)94(Po)a6oc(2&)8kk{mm(4V)f(dB)c(16d)agq(a{)74s(2w)6a({Y)8(8t)8o(49)6)j5(c3)6G({V)7(3a)7Y)L7(dB)b(1a^)9%%a)04Aq{8{{{!(3h)64(j6)82(rp)7(i-)a(aQ)6(10J)a(16)6(v4)aIq(dB)9)0aq(s4)d(IE)aY0(Af)7(rt)fu(6%)bY{6{{G{G(7a)bo(43)8(g^)7)%5(14f)8(kC)9(=-)6{g(yG)9(bw)8Y{{4(1p)e2{)A64(mH)7(1gT)8(2N)78{{{o{6{6(K2)b{{{C(ri)a(sD)9(ss)72(TA)6u(F.)aG(i9)j{!(Iw)c(EF)gA)b72Y2Yc{cG0G{YaAY{A(7$)72(2e)6u(as)bo(2P)7(9a)b(7C)ag8G{G{Go(13k)8(=P)h(aW)6A(u1)9(fA)auuuc)08(I^)8(4+)7g48A8c(dB)92{{6)15(1O)8)Fa(?O)agc(.^)9o(8!)7(hA)6)08{(2P)6(5c)7)65(4m)c2(3c)7)ib(83)7gSg(4w)6)rb)V8(9t)6(I=)j22(7})bg(?L)aE{G{GYG{)1faGa(kL)6Y{66{66(js)7(1o)nYY(3z)9(d=)7(m=)98{{{2{GY(7s)9)08{c(1io)8K(5J)b(c7)7o(w!)6)1e(ZA)a2c{g4!{{{4!G{4{8G{{{ggGg(VH)a(iI)7cG(14y)7S(1j9)fg(49)8(1m)cY{4(90)8)^4!0IcY(5E)6g(dm)7(FL)6(1p)f(2P)6ug8a8a8Aa(x9)ai(2O)6Y!I)04(9l)a(43)8c)74(1Z)68)48(82)74(bC)a(2P)bE{s(fd)82{(RU)7(bT)8!III(uW)6(Ld)c)e9(1vv)a(7E)9c{Y2(aa)6(1p)6aaaa{Ao2%a88aaC4(fT)9(S4)7Y!!eeqq(8^)aG(fi)7(1e6)9S{{i(e?)66(Gf)b(5q)a(2P)9g{YYYGGo(18I)a(ex)9(1p)7(2A)8(43)7a(bG)b(3v)6(wf)a{{6aaG(1p)9g)o8Yccc2(uj)6(=Y)7)n5YY!888q(vT)7(h3)8(35)6)T6(4k)7(ZM)6(v8)7(1p)64(1p)7{igig{{c(Ct)6(Po)7(1Cz)7(cz)68)04(1p)62S(1Eb)a(eS)62{8(16G)7(5t)a2{2(43)6Y{cGc2o(e!)9cG(i4)ao{0000(zJ)6YY)5a{a4%4s4(1e-)7(pB)d(ys)84{{AG{o{6ccc2222(2t)9(1p)b(2P)68{{(r?)7{aa{cc{cc(BL)6%(BE)a(17)7(zo)7{0(}%)e(zY)6G{o{{o(1Ai)8(2P)9A{8(Z&)8(5z)7(m1)c%(N^)a4(X{)6(Yt)9c)39Y(dA)aYoc4{o(a=)7{cG{gSAa{{oo{o(155)a(1p)a{G(2P)6(3?)a(1n)accoca(!m)8(1n)74(TB)6(1m)7(cp)8(1p)6{S(Rq)8)07(eX)7Y(Yq)6(Ln)6(8b)8co(2T)8(Bf)6(2P)7o!(4h)7Y(1Kl)9(C^)8(2a)6o(1d)a)1d(Yq)7G{{{a{!Y(%2)c(3})a(2-)66!{2o2(xp)cc2o(a+)6G)18(iQ)e)0b%{4{4{o{o!G{{c{6(e^)82(^=)a{{{(2P)6(kT)b{c2c(1p)e(1d)f(kG)b2(LT)7a{).4c2{c(1OR)c(a&)9c{8(u&)7(?^)7(xr)8{{c(ab)b)L46c)38(y$)7!0ag(I$)7!{{o(43)660(12z)dY{{Y(38)6)0lca(7f)8Y0{g(9M)6)38(an)62a(1o)6(2P)68a(jc)eg(mf)8{{g(6k)k(58)9og{{{c(1d)6g)356(nk)b!(LF)8!(Tq)6(1FG)b2(1Cd)a{G{o(1p)rcSG0G($U)6(Th)8(Dt)8Y6g(VM)80(1fy)h)c9(1O1)9(1l!)b(2j)8)346a{{o(o.)6g(hU)6g{{6(2N)966{!(o=)6c(19U)7)A4(k%)e(kF)72{2{{S{i)3h{gcG0(1Wy)82(DR)8(2W)7(}%)b{Ao(1Fh)c(vt)b4(xz)a8(1I7)7(2V)c)$8(1LE)d(89)8(1Qt)d)05;;";
				}
				let mut grid = import(level).unwrap();
				
				render(&mut grid, ticks);

				if let Err(why) = command.defer(&ctx.http).await {
					println!("Cannot defer interaction: {}", why);
				};
				
				if let Err(why) = command.create_followup_message(&ctx.http, |response| {
					response
						//.kind(InteractionResponseType::ChannelMessageWithSource)
						//.interaction_response_data(|message| {
						//	message
								.content(format!("Time: {}ms", now.elapsed().as_millis()))
								/*.embed(|embed| {
									embed
										.title("Render")
										.description("")
										.color(0x43FF19)
										.image("attachment://render.mp4")
								})*/
								.add_file(Path::new("render.mp4"))
					//	})
				}).await {
					println!("Cannot response to slash command: {}", why);
				}
			}
		}
	}

	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is connected", ready.user.name);

		let guild_id = GuildId(
			env::var("GUILD_ID")
				.expect("Expected GUILD_ID in env")
				.parse()
				.expect("GUILD_ID must be an integer")
		);

		let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
			commands.create_application_command(|command| {
				command.name("preview")
			    	.description("Preview a level code (V3 only)")
			    	.create_option(|option| {
			    		option
			    			.kind(CommandOptionType::String)
			    			.name("level")
			    			.description("The level code")
			    			.required(true)
			    	})
			    	.create_option(|option| {
			    		option
			    			.kind(CommandOptionType::Integer)
			    			.name("ticks")
			    			.description("How many ticks to render")
			    			.required(true)
			    			.min_int_value(1)
			    			.max_int_value(100)
			    	})
			 })
		}).await;

		println!("I now have the following guild slash commands: {:#?}", commands);
	}
}

/*fn print_grid(grid: Grid) {
	grid.for_each(|x, y, cell| {
		if let Some(cell) = cell {
			print!("{} ", cell.id());
		} else {
			print!(". ");
		}
		if x == (grid.width-1).try_into().unwrap() {
			print!("\n");
		}
	});
}*/

#[tokio::main]
async fn main() {
	let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in env");

	let now = Instant::now();
	
	let level = "V3;1q;1q;2{{{g{)0g4g{{4)a7)la2{{{G)d9)94)Vdu{{{8)-b{e)b9YG{2)Z4)0f6(1h)c2c)a7SS)l9g)f5)44i)g6k{4)f6{2{4o)s8{{{{Y8i{c4Y{{aa{6a666c)l4o{{2e{)14{{e{o{22cccc{cG(2P)64{{!(2Z)82(2^)6G{{0{{6g4(3h)6{{{6eAe(3F)8Y{g{a{g(3D)9S)9dY{{S)!5(2r)7aa(4m)9ii(2l)6GG{2(2x)a6{{G{{2(1v)6)G6aaaa!Aa)74(2P)e)u5(4G)64{c{g(1p)6a{a)?44{4(1$)9Y2o{c{cG(4D)78{4(58)bG)^68{{{!(1p)f{{{a(6T)cgY(2P)6a)m4)i4)f4{a{2{2{66(3%)6g)k4g{S{{c(3b)7(1m)7(7k)7(2P)6{{{{6a{{6{6ssss)w6(8f)ai{a{(41)6)36)N8c{{c)15{!0aa8g{{M(8e)c{{6Yg{2(1p)b(9c)98(7E)7(7U)9{{Yaa{8{{o)W6aa{)O4(3?)6Y(9&)7ag(8.)cc4)G42{i{{4(1p)b8)04)y7ea(52)9)99S{4{4(a+)7aa(1g)72(2P)6Y{ga{!A!(6P)622{o(1A)66a)0eo(2V)b(2R)b{a{cG0G(3I)6{{{4(97)8(c})fi)f7(4V)6(3x)6!888!(5t)b4c{{{2(6m)6(4e)84{2{{a{{88a(74)7)&42(3P)8G{{k(5t)7)u5c)04o{{!o(94)744{2)&5(5t)9(du)a4i2{{{0{Ae0e{aY)&6)B4s{{c)28(dx)8)X90YoYo)I4!)i7g{6{G(87)b(7M)6(5t)6eA{0(2P)d(1p)8gS{{6c)27G{{2)74(2P)62cY(2P)6(1k)6(3M)6o)36a{agg)Z6{{(6U)64i4{2i2(7F)7{a{{{g8a)04{s(3A)6)28(5j)a{4(7V)6(e-)8(6k)7Y{(3C)6Y)Z5a(iA)84{{e(ce)782{6cca)R4i{{a(5&)6(2Z)6(5G)6o)540)T60{g(1q)6{{44(2P)cc2a(1E)64c(9z)a4S4{2S(fO)6c24)S58(dY)9(1p)9(5w)7(8l)7g{Y0(e1)7{{)T4(7X)7Y{{4G(1E)7(ig)7(3&)b(3+)6{KY(9&)7!A!)k4a)27(7{)7)0b0)U6(9%)8Y(87)9S{8{24{{c%A%(5t)ga{{{gi(4O)7(aW)7{2)28(1o)b)a60(h2)aG(53)6(6T)6!G{0Y)C5%A%(nD)90{A)L9a(b3)7S{2{g)Z5)26(2P)82{4{(5d)60)C6(bP)b(43)6!G{0YYY(33)6)A56{{{4S(49)74{2{8g8(5S)7A(7c)6)26c{c{{oG)l6{{o(m-)8(jT)8c(6%)622G{{{{ccY(gP)9{(e=)ca{{6{G(oh)7{{{A)sc{c(58)a)le(rR)e(dn)8(9G)6(sc)b(fo)6a{(3$)7)R8(1p)6(gh)6c{c{a(s+)c(1C)c2{6!8g(2P)6Y(4x)6(1p)6(kq)b(i-)6)g4(1n)8GGG(3T)7(2t)b(58)6(u5)e)08(e.)6(5E)c(1p)9(2U)b(c=)7{(5t)6(57)6(iT)a)94K)Aa)qaY)+6!{Y(aJ)6Y(bM)7aaa{{a)C6G(1u)g(jZ)82(8y)6(3q)6A)c4E(3!)a(5E)h!(3-)84{!G{{g%%%%{6ac{S(6a)f6G{6aaG)748(8H)b{(7^)6E(ma)c(1p)i(45)b2(d4)62{6ccG{{8{a8a8(5F)64(3z)7)44)34i{{0{aY{Ya(9J)6a0siK%asS)44)9dg{!{o{{{Y(zy)9%(9$)6c2)^4g(wC)a{YYY(eW)90{{{go888gog888(jV)7(3i)8666eee(As)a(2P)9(wZ)8a)bbS(A=)b08(sD)6(2&)9{{oooA{0ooo)n8(vY)9(A+)d(1p)8Y(ha)6{{{%(6I)72{)Q4(4Z)96i{gYS(1p)8(72)j4)A48c(67)7c{688G{U{U(6T)a)X5!)76(CR)ac{e(3m)90{{8(1p)8i{{{g(n-)92(d2)74{iS(v9)8(7E)e(2P)7(iA)6(5t)8{{2g{GGSG)08)9a%2{)q4(fm)6o(b!)72{c{{4o{8!8!2c{{6!(gT)68YU+{{a(6T)6(mq)9(fe)94Y)U4(2P)e)r4)J4(2h)6(Gk)b(5l)94{!)v5(9h)8)74a(87)9(z2)c(dK)9(hM)7a8a(aE)6c)d4(1n)6(57)7(3S)a)97(1j)6)56W)74a{a(1D)7a6A68c(Ep)8)L4(1p)8o(4D)60{{c{62{o{2{6)Q4(7!)8(jd)a(1p)hG{(4b)6!4)H4(8k)6(3v)8g{GGG(fl)7o{{(2P)9(43)8(3&)e(l1)6)e8Y(1b)bW{W(li)b(4n)8)F7(np)a(18)9(eI)6Y{o(Me)b(5v)a6o{o(zn)6(25)a)04+(h!)aY(1p)6g(1p)7)X6)o4)H7c{{0(h1)7o%)046A6(jO)g(2d)7)&5(j0)7(h5)e!8)O44{c(5{)7(4X)7g{A(rA)7{0(1B)8A)74%)l6(MU)9)0r)t86!c!{{{I(rK)7(2P)a(e?)7(dI)94(b^)8(1%)6(4x)9(4d)7000(dD)8{2)$666Y(57)6(2P)6oo(2P)g(8.)8(JQ)8(w9)i(uE)6(2T)c(cQ)8oo6)06YYY{{{g6Ai(4S)6(5t)e)U5G(7Z)b(CV)6K0KK)h6(3^)d(29)6(bE)a(1O)6o4ccYY{{g(66)8c(bI)8(2P)9)89)N4(1o)6(iE)g)FeY{4(it)6oG(ju)6{c{YY{{4(4C)74(87)h4{{q(26)9(1o)8c2{62(c4)6(2Q)8(aW)9{Y{o)w4cco(gP)7(1p)7)=88(5t)fg)44q{S{{4{6{{4(2P)9%(R$)6cc(oK)7(IN)7g{e{Y{{2oc{oo{(1q)6(1p)9(2t)66o(87)f%(1p)6(cp)a(1o)6A)g566{oo)94A{{4cc2)85eY(oe)boG{{{occ4(1p)8(1d)6(2P)7Y(3D)6%(Ff)7(6u)a(5r)jo{o{{o6G{g{{{e(sL)7(H5)7G{{{ooGGY{{{2cccY(3m)7)u64(rd)9(Bj)8Y!A!(8h)7(5t)6($f)c(IF)8e(D0)d(46)7(8s)aYA{4{c(aE)7)06K(1y)6(RJ)7g(12)6)u4(k%)8(cz)9(9d)6(5t)6(eB)6{44{2{oo2{2(Qp)aY{{I(bM)f6%%%{{{S(C-)d4(E0)7ee(%s)b)x5A(2P)6(gE)96c{oo{{Go(JV)9Y)F5c(2j)6(NI)6%(3{)6So{{q0q(5^)6)45g{{iG{{{i{4{o(kF)8g{{{e{e{!{{4{6cY2{o{oo{Y{(1p)72)Y56c2A24c{CE)0444(5t)dg(av)7(y})8ee(5t)7A)?5{{o0(1p)9{86(1L)6(1p)c(d6)820(hc)7(x=)a(p.)e4(5X)a6(bt)7A(hO)b(.c)662(%t)6(1p)gA{{{4c2S(1p)768GA(1p)d4(&G)9(Jb)6{{G{{a(9b)9(lr)h(2l)6(1p)bA(4C)68c2A(1p)7068G(Pk)7{g{2{{2a(JD)io(cQ)74{o(Eu)c(eJ)7)Q46(1p)a(4X)6o(6T)9(AL)a(2i)6cg{2(3^)72)j4($0)6(J8)b(43)9Ye(%I)9(p})6c(1p)8(oQ)9{4(7S)7g4444msssAsc{{aq(5A)d42(bX)d{{g(wR)72(7b)9(mX)7o(hW)6(Y=)6Y{)?4!(?K)7(2b)62i2{s(&j)6qa)04(1f)7!($a)6Yao(?r)7(5K)7e(l%)b(6o)6(xM)6o{Y{{Y!YYYq(cB)62(Km)6aa)$5Ss(4c)6(2P)cM(6A)h{{o{{g(112)c{G{4(1o)6(2P)9c{Y{c{{I(1p)66(2R)a(Rm)8)07(eQ)8(X})6(2J)6GY(sy)c8)O6uuuu6o{{GGGY6(4h)6(43)66o(43)8(}T)6(qs)9{{(2P)cS(ay)6(13Y)9(CE)c)i76)04o{{Y2uo2(dt)6(5t)9(mM)7(3f)72)!5Y2oo)s6(.M)76i)49gg{{!!A!c(3?)7(4t)76o)04{{{6(2T)84(87)d(tK)h66{{{q{42(6.)6Y{M(2P)9(7$)9c(1i)62)O8ou(Eb)i!)?5{48(x5)gc{c6c(1p)6a4{s(AZ)c(gr)c(ni)f6ou(XX)6(Eb)84(1p)a8{(16x)b(1p)f224{(8k)6(1t)b(1p)9G(O1)eou{o{6{{{G{GY{6)a5(43)7(F%)8(p+)84(1p)b)04(1p)6EI(4n)6{g{YY(5m)9(12V)7G(s0)8($n)8624)I4g{{ggg)E4q{4Y{G{{Y(1})8(1E)8q{gKY{s(9V)6)s6S{{Yc++(UL)94(Po)a6oc(2&)8kk{mm(4V)f(dB)c(16d)agq(a{)74s(2w)6a({Y)8(8t)8o(49)6)j5(c3)6G({V)7(3a)7Y)L7(dB)b(1a^)9%%a)04Aq{8{{{!(3h)64(j6)82(rp)7(i-)a(aQ)6(10J)a(16)6(v4)aIq(dB)9)0aq(s4)d(IE)aY0(Af)7(rt)fu(6%)bY{6{{G{G(7a)bo(43)8(g^)7)%5(14f)8(kC)9(=-)6{g(yG)9(bw)8Y{{4(1p)e2{)A64(mH)7(1gT)8(2N)78{{{o{6{6(K2)b{{{C(ri)a(sD)9(ss)72(TA)6u(F.)aG(i9)j{!(Iw)c(EF)gA)b72Y2Yc{cG0G{YaAY{A(7$)72(2e)6u(as)bo(2P)7(9a)b(7C)ag8G{G{Go(13k)8(=P)h(aW)6A(u1)9(fA)auuuc)08(I^)8(4+)7g48A8c(dB)92{{6)15(1O)8)Fa(?O)agc(.^)9o(8!)7(hA)6)08{(2P)6(5c)7)65(4m)c2(3c)7)ib(83)7gSg(4w)6)rb)V8(9t)6(I=)j22(7})bg(?L)aE{G{GYG{)1faGa(kL)6Y{66{66(js)7(1o)nYY(3z)9(d=)7(m=)98{{{2{GY(7s)9)08{c(1io)8K(5J)b(c7)7o(w!)6)1e(ZA)a2c{g4!{{{4!G{4{8G{{{ggGg(VH)a(iI)7cG(14y)7S(1j9)fg(49)8(1m)cY{4(90)8)^4!0IcY(5E)6g(dm)7(FL)6(1p)f(2P)6ug8a8a8Aa(x9)ai(2O)6Y!I)04(9l)a(43)8c)74(1Z)68)48(82)74(bC)a(2P)bE{s(fd)82{(RU)7(bT)8!III(uW)6(Ld)c)e9(1vv)a(7E)9c{Y2(aa)6(1p)6aaaa{Ao2%a88aaC4(fT)9(S4)7Y!!eeqq(8^)aG(fi)7(1e6)9S{{i(e?)66(Gf)b(5q)a(2P)9g{YYYGGo(18I)a(ex)9(1p)7(2A)8(43)7a(bG)b(3v)6(wf)a{{6aaG(1p)9g)o8Yccc2(uj)6(=Y)7)n5YY!888q(vT)7(h3)8(35)6)T6(4k)7(ZM)6(v8)7(1p)64(1p)7{igig{{c(Ct)6(Po)7(1Cz)7(cz)68)04(1p)62S(1Eb)a(eS)62{8(16G)7(5t)a2{2(43)6Y{cGc2o(e!)9cG(i4)ao{0000(zJ)6YY)5a{a4%4s4(1e-)7(pB)d(ys)84{{AG{o{6ccc2222(2t)9(1p)b(2P)68{{(r?)7{aa{cc{cc(BL)6%(BE)a(17)7(zo)7{0(}%)e(zY)6G{o{{o(1Ai)8(2P)9A{8(Z&)8(5z)7(m1)c%(N^)a4(X{)6(Yt)9c)39Y(dA)aYoc4{o(a=)7{cG{gSAa{{oo{o(155)a(1p)a{G(2P)6(3?)a(1n)accoca(!m)8(1n)74(TB)6(1m)7(cp)8(1p)6{S(Rq)8)07(eX)7Y(Yq)6(Ln)6(8b)8co(2T)8(Bf)6(2P)7o!(4h)7Y(1Kl)9(C^)8(2a)6o(1d)a)1d(Yq)7G{{{a{!Y(%2)c(3})a(2-)66!{2o2(xp)cc2o(a+)6G)18(iQ)e)0b%{4{4{o{o!G{{c{6(e^)82(^=)a{{{(2P)6(kT)b{c2c(1p)e(1d)f(kG)b2(LT)7a{).4c2{c(1OR)c(a&)9c{8(u&)7(?^)7(xr)8{{c(ab)b)L46c)38(y$)7!0ag(I$)7!{{o(43)660(12z)dY{{Y(38)6)0lca(7f)8Y0{g(9M)6)38(an)62a(1o)6(2P)68a(jc)eg(mf)8{{g(6k)k(58)9og{{{c(1d)6g)356(nk)b!(LF)8!(Tq)6(1FG)b2(1Cd)a{G{o(1p)rcSG0G($U)6(Th)8(Dt)8Y6g(VM)80(1fy)h)c9(1O1)9(1l!)b(2j)8)346a{{o(o.)6g(hU)6g{{6(2N)966{!(o=)6c(19U)7)A4(k%)e(kF)72{2{{S{i)3h{gcG0(1Wy)82(DR)8(2W)7(}%)b{Ao(1Fh)c(vt)b4(xz)a8(1I7)7(2V)c)$8(1LE)d(89)8(1Qt)d)05;;";
	let mut grid = import(level).unwrap();
	render(&mut grid, 11);

	println!("render() + import(): {}millis", now.elapsed().as_millis());

	let mut client = Client::builder(token, GatewayIntents::empty())
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}





	/*let args: Vec<String> = std::env::args().collect();
	let mut grid = import(&args[1]).unwrap();
	let ticks: usize = args[2].parse().unwrap();

	preview(grid, ticks);*/
}




	
	//let mut grid = import("V3;1q;1q;2{{{g{)0g4g{{4)a7)la2{{{G)d9)94)Vdu{{{8)-b{e)b9YG{2)Z4)0f6(1h)c2c)a7SS)l9g)f5)44i)g6k{4)f6{2{4o)s8{{{{Y8i{c4Y{{aa{6a666c)l4o{{2e{)14{{e{o{22cccc{cG(2P)64{{!(2Z)82(2^)6G{{0{{6g4(3h)6{{{6eAe(3F)8Y{g{a{g(3D)9S)9dY{{S)!5(2r)7aa(4m)9ii(2l)6GG{2(2x)a6{{G{{2(1v)6)G6aaaa!Aa)74(2P)e)u5(4G)64{c{g(1p)6a{a)?44{4(1$)9Y2o{c{cG(4D)78{4(58)bG)^68{{{!(1p)f{{{a(6T)cgY(2P)6a)m4)i4)f4{a{2{2{66(3%)6g)k4g{S{{c(3b)7(1m)7(7k)7(2P)6{{{{6a{{6{6ssss)w6(8f)ai{a{(41)6)36)N8c{{c)15{!0aa8g{{M(8e)c{{6Yg{2(1p)b(9c)98(7E)7(7U)9{{Yaa{8{{o)W6aa{)O4(3?)6Y(9&)7ag(8.)cc4)G42{i{{4(1p)b8)04)y7ea(52)9)99S{4{4(a+)7aa(1g)72(2P)6Y{ga{!A!(6P)622{o(1A)66a)0eo(2V)b(2R)b{a{cG0G(3I)6{{{4(97)8(c})fi)f7(4V)6(3x)6!888!(5t)b4c{{{2(6m)6(4e)84{2{{a{{88a(74)7)&42(3P)8G{{k(5t)7)u5c)04o{{!o(94)744{2)&5(5t)9(du)a4i2{{{0{Ae0e{aY)&6)B4s{{c)28(dx)8)X90YoYo)I4!)i7g{6{G(87)b(7M)6(5t)6eA{0(2P)d(1p)8gS{{6c)27G{{2)74(2P)62cY(2P)6(1k)6(3M)6o)36a{agg)Z6{{(6U)64i4{2i2(7F)7{a{{{g8a)04{s(3A)6)28(5j)a{4(7V)6(e-)8(6k)7Y{(3C)6Y)Z5a(iA)84{{e(ce)782{6cca)R4i{{a(5&)6(2Z)6(5G)6o)540)T60{g(1q)6{{44(2P)cc2a(1E)64c(9z)a4S4{2S(fO)6c24)S58(dY)9(1p)9(5w)7(8l)7g{Y0(e1)7{{)T4(7X)7Y{{4G(1E)7(ig)7(3&)b(3+)6{KY(9&)7!A!)k4a)27(7{)7)0b0)U6(9%)8Y(87)9S{8{24{{c%A%(5t)ga{{{gi(4O)7(aW)7{2)28(1o)b)a60(h2)aG(53)6(6T)6!G{0Y)C5%A%(nD)90{A)L9a(b3)7S{2{g)Z5)26(2P)82{4{(5d)60)C6(bP)b(43)6!G{0YYY(33)6)A56{{{4S(49)74{2{8g8(5S)7A(7c)6)26c{c{{oG)l6{{o(m-)8(jT)8c(6%)622G{{{{ccY(gP)9{(e=)ca{{6{G(oh)7{{{A)sc{c(58)a)le(rR)e(dn)8(9G)6(sc)b(fo)6a{(3$)7)R8(1p)6(gh)6c{c{a(s+)c(1C)c2{6!8g(2P)6Y(4x)6(1p)6(kq)b(i-)6)g4(1n)8GGG(3T)7(2t)b(58)6(u5)e)08(e.)6(5E)c(1p)9(2U)b(c=)7{(5t)6(57)6(iT)a)94K)Aa)qaY)+6!{Y(aJ)6Y(bM)7aaa{{a)C6G(1u)g(jZ)82(8y)6(3q)6A)c4E(3!)a(5E)h!(3-)84{!G{{g%%%%{6ac{S(6a)f6G{6aaG)748(8H)b{(7^)6E(ma)c(1p)i(45)b2(d4)62{6ccG{{8{a8a8(5F)64(3z)7)44)34i{{0{aY{Ya(9J)6a0siK%asS)44)9dg{!{o{{{Y(zy)9%(9$)6c2)^4g(wC)a{YYY(eW)90{{{go888gog888(jV)7(3i)8666eee(As)a(2P)9(wZ)8a)bbS(A=)b08(sD)6(2&)9{{oooA{0ooo)n8(vY)9(A+)d(1p)8Y(ha)6{{{%(6I)72{)Q4(4Z)96i{gYS(1p)8(72)j4)A48c(67)7c{688G{U{U(6T)a)X5!)76(CR)ac{e(3m)90{{8(1p)8i{{{g(n-)92(d2)74{iS(v9)8(7E)e(2P)7(iA)6(5t)8{{2g{GGSG)08)9a%2{)q4(fm)6o(b!)72{c{{4o{8!8!2c{{6!(gT)68YU+{{a(6T)6(mq)9(fe)94Y)U4(2P)e)r4)J4(2h)6(Gk)b(5l)94{!)v5(9h)8)74a(87)9(z2)c(dK)9(hM)7a8a(aE)6c)d4(1n)6(57)7(3S)a)97(1j)6)56W)74a{a(1D)7a6A68c(Ep)8)L4(1p)8o(4D)60{{c{62{o{2{6)Q4(7!)8(jd)a(1p)hG{(4b)6!4)H4(8k)6(3v)8g{GGG(fl)7o{{(2P)9(43)8(3&)e(l1)6)e8Y(1b)bW{W(li)b(4n)8)F7(np)a(18)9(eI)6Y{o(Me)b(5v)a6o{o(zn)6(25)a)04+(h!)aY(1p)6g(1p)7)X6)o4)H7c{{0(h1)7o%)046A6(jO)g(2d)7)&5(j0)7(h5)e!8)O44{c(5{)7(4X)7g{A(rA)7{0(1B)8A)74%)l6(MU)9)0r)t86!c!{{{I(rK)7(2P)a(e?)7(dI)94(b^)8(1%)6(4x)9(4d)7000(dD)8{2)$666Y(57)6(2P)6oo(2P)g(8.)8(JQ)8(w9)i(uE)6(2T)c(cQ)8oo6)06YYY{{{g6Ai(4S)6(5t)e)U5G(7Z)b(CV)6K0KK)h6(3^)d(29)6(bE)a(1O)6o4ccYY{{g(66)8c(bI)8(2P)9)89)N4(1o)6(iE)g)FeY{4(it)6oG(ju)6{c{YY{{4(4C)74(87)h4{{q(26)9(1o)8c2{62(c4)6(2Q)8(aW)9{Y{o)w4cco(gP)7(1p)7)=88(5t)fg)44q{S{{4{6{{4(2P)9%(R$)6cc(oK)7(IN)7g{e{Y{{2oc{oo{(1q)6(1p)9(2t)66o(87)f%(1p)6(cp)a(1o)6A)g566{oo)94A{{4cc2)85eY(oe)boG{{{occ4(1p)8(1d)6(2P)7Y(3D)6%(Ff)7(6u)a(5r)jo{o{{o6G{g{{{e(sL)7(H5)7G{{{ooGGY{{{2cccY(3m)7)u64(rd)9(Bj)8Y!A!(8h)7(5t)6($f)c(IF)8e(D0)d(46)7(8s)aYA{4{c(aE)7)06K(1y)6(RJ)7g(12)6)u4(k%)8(cz)9(9d)6(5t)6(eB)6{44{2{oo2{2(Qp)aY{{I(bM)f6%%%{{{S(C-)d4(E0)7ee(%s)b)x5A(2P)6(gE)96c{oo{{Go(JV)9Y)F5c(2j)6(NI)6%(3{)6So{{q0q(5^)6)45g{{iG{{{i{4{o(kF)8g{{{e{e{!{{4{6cY2{o{oo{Y{(1p)72)Y56c2A24c{CE)0444(5t)dg(av)7(y})8ee(5t)7A)?5{{o0(1p)9{86(1L)6(1p)c(d6)820(hc)7(x=)a(p.)e4(5X)a6(bt)7A(hO)b(.c)662(%t)6(1p)gA{{{4c2S(1p)768GA(1p)d4(&G)9(Jb)6{{G{{a(9b)9(lr)h(2l)6(1p)bA(4C)68c2A(1p)7068G(Pk)7{g{2{{2a(JD)io(cQ)74{o(Eu)c(eJ)7)Q46(1p)a(4X)6o(6T)9(AL)a(2i)6cg{2(3^)72)j4($0)6(J8)b(43)9Ye(%I)9(p})6c(1p)8(oQ)9{4(7S)7g4444msssAsc{{aq(5A)d42(bX)d{{g(wR)72(7b)9(mX)7o(hW)6(Y=)6Y{)?4!(?K)7(2b)62i2{s(&j)6qa)04(1f)7!($a)6Yao(?r)7(5K)7e(l%)b(6o)6(xM)6o{Y{{Y!YYYq(cB)62(Km)6aa)$5Ss(4c)6(2P)cM(6A)h{{o{{g(112)c{G{4(1o)6(2P)9c{Y{c{{I(1p)66(2R)a(Rm)8)07(eQ)8(X})6(2J)6GY(sy)c8)O6uuuu6o{{GGGY6(4h)6(43)66o(43)8(}T)6(qs)9{{(2P)cS(ay)6(13Y)9(CE)c)i76)04o{{Y2uo2(dt)6(5t)9(mM)7(3f)72)!5Y2oo)s6(.M)76i)49gg{{!!A!c(3?)7(4t)76o)04{{{6(2T)84(87)d(tK)h66{{{q{42(6.)6Y{M(2P)9(7$)9c(1i)62)O8ou(Eb)i!)?5{48(x5)gc{c6c(1p)6a4{s(AZ)c(gr)c(ni)f6ou(XX)6(Eb)84(1p)a8{(16x)b(1p)f224{(8k)6(1t)b(1p)9G(O1)eou{o{6{{{G{GY{6)a5(43)7(F%)8(p+)84(1p)b)04(1p)6EI(4n)6{g{YY(5m)9(12V)7G(s0)8($n)8624)I4g{{ggg)E4q{4Y{G{{Y(1})8(1E)8q{gKY{s(9V)6)s6S{{Yc++(UL)94(Po)a6oc(2&)8kk{mm(4V)f(dB)c(16d)agq(a{)74s(2w)6a({Y)8(8t)8o(49)6)j5(c3)6G({V)7(3a)7Y)L7(dB)b(1a^)9%%a)04Aq{8{{{!(3h)64(j6)82(rp)7(i-)a(aQ)6(10J)a(16)6(v4)aIq(dB)9)0aq(s4)d(IE)aY0(Af)7(rt)fu(6%)bY{6{{G{G(7a)bo(43)8(g^)7)%5(14f)8(kC)9(=-)6{g(yG)9(bw)8Y{{4(1p)e2{)A64(mH)7(1gT)8(2N)78{{{o{6{6(K2)b{{{C(ri)a(sD)9(ss)72(TA)6u(F.)aG(i9)j{!(Iw)c(EF)gA)b72Y2Yc{cG0G{YaAY{A(7$)72(2e)6u(as)bo(2P)7(9a)b(7C)ag8G{G{Go(13k)8(=P)h(aW)6A(u1)9(fA)auuuc)08(I^)8(4+)7g48A8c(dB)92{{6)15(1O)8)Fa(?O)agc(.^)9o(8!)7(hA)6)08{(2P)6(5c)7)65(4m)c2(3c)7)ib(83)7gSg(4w)6)rb)V8(9t)6(I=)j22(7})bg(?L)aE{G{GYG{)1faGa(kL)6Y{66{66(js)7(1o)nYY(3z)9(d=)7(m=)98{{{2{GY(7s)9)08{c(1io)8K(5J)b(c7)7o(w!)6)1e(ZA)a2c{g4!{{{4!G{4{8G{{{ggGg(VH)a(iI)7cG(14y)7S(1j9)fg(49)8(1m)cY{4(90)8)^4!0IcY(5E)6g(dm)7(FL)6(1p)f(2P)6ug8a8a8Aa(x9)ai(2O)6Y!I)04(9l)a(43)8c)74(1Z)68)48(82)74(bC)a(2P)bE{s(fd)82{(RU)7(bT)8!III(uW)6(Ld)c)e9(1vv)a(7E)9c{Y2(aa)6(1p)6aaaa{Ao2%a88aaC4(fT)9(S4)7Y!!eeqq(8^)aG(fi)7(1e6)9S{{i(e?)66(Gf)b(5q)a(2P)9g{YYYGGo(18I)a(ex)9(1p)7(2A)8(43)7a(bG)b(3v)6(wf)a{{6aaG(1p)9g)o8Yccc2(uj)6(=Y)7)n5YY!888q(vT)7(h3)8(35)6)T6(4k)7(ZM)6(v8)7(1p)64(1p)7{igig{{c(Ct)6(Po)7(1Cz)7(cz)68)04(1p)62S(1Eb)a(eS)62{8(16G)7(5t)a2{2(43)6Y{cGc2o(e!)9cG(i4)ao{0000(zJ)6YY)5a{a4%4s4(1e-)7(pB)d(ys)84{{AG{o{6ccc2222(2t)9(1p)b(2P)68{{(r?)7{aa{cc{cc(BL)6%(BE)a(17)7(zo)7{0(}%)e(zY)6G{o{{o(1Ai)8(2P)9A{8(Z&)8(5z)7(m1)c%(N^)a4(X{)6(Yt)9c)39Y(dA)aYoc4{o(a=)7{cG{gSAa{{oo{o(155)a(1p)a{G(2P)6(3?)a(1n)accoca(!m)8(1n)74(TB)6(1m)7(cp)8(1p)6{S(Rq)8)07(eX)7Y(Yq)6(Ln)6(8b)8co(2T)8(Bf)6(2P)7o!(4h)7Y(1Kl)9(C^)8(2a)6o(1d)a)1d(Yq)7G{{{a{!Y(%2)c(3})a(2-)66!{2o2(xp)cc2o(a+)6G)18(iQ)e)0b%{4{4{o{o!G{{c{6(e^)82(^=)a{{{(2P)6(kT)b{c2c(1p)e(1d)f(kG)b2(LT)7a{).4c2{c(1OR)c(a&)9c{8(u&)7(?^)7(xr)8{{c(ab)b)L46c)38(y$)7!0ag(I$)7!{{o(43)660(12z)dY{{Y(38)6)0lca(7f)8Y0{g(9M)6)38(an)62a(1o)6(2P)68a(jc)eg(mf)8{{g(6k)k(58)9og{{{c(1d)6g)356(nk)b!(LF)8!(Tq)6(1FG)b2(1Cd)a{G{o(1p)rcSG0G($U)6(Th)8(Dt)8Y6g(VM)80(1fy)h)c9(1O1)9(1l!)b(2j)8)346a{{o(o.)6g(hU)6g{{6(2N)966{!(o=)6c(19U)7)A4(k%)e(kF)72{2{{S{i)3h{gcG0(1Wy)82(DR)8(2W)7(}%)b{Ao(1Fh)c(vt)b4(xz)a8(1I7)7(2V)c)$8(1LE)d(89)8(1Qt)d)05;;").unwrap();
	/*print_grid(grid);
	for t in 0..10 {
		update(&mut grid);
		print_grid(grid.clone());
		print!("\n");
	}*/

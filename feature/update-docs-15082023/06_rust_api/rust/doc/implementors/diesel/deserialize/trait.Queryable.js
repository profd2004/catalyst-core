(function() {var implementors = {
"vit_servicing_station_lib":[["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/snapshot/struct.Snapshot.html\" title=\"struct vit_servicing_station_lib::db::models::snapshot::Snapshot\">Snapshot</a><span class=\"where fmt-newline\">where\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Integer, Text, Text, Text, Text, Text, BigInt, Text, Text, BigInt, Text, Text, Text, Text, Binary, Array&lt;Text&gt;, Integer, Nullable&lt;Text&gt;, Integer, BigInt, BigInt, BigInt, Text, Text, Integer, Text, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, BigInt, Text, Text), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/proposals/struct.Proposal.html\" title=\"struct vit_servicing_station_lib::db::models::proposals::Proposal\">Proposal</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>: FromSql&lt;Integer, DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;Array&lt;Text&gt;, DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/goals/struct.Goal.html\" title=\"struct vit_servicing_station_lib::db::models::goals::Goal\">Goal</a><span class=\"where fmt-newline\">where\n    (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/groups/struct.Group.html\" title=\"struct vit_servicing_station_lib::db::models::groups::Group\">Group</a><span class=\"where fmt-newline\">where\n    (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;__ST, __DB&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"enum\" href=\"vit_servicing_station_lib/db/models/community_advisors_reviews/enum.ReviewRanking.html\" title=\"enum vit_servicing_station_lib::db::models::community_advisors_reviews::ReviewRanking\">ReviewRanking</a><span class=\"where fmt-newline\">where\n    __DB: Backend,\n    Self: FromSql&lt;__ST, __DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/vote/struct.Vote.html\" title=\"struct vit_servicing_station_lib::db::models::vote::Vote\">Vote</a><span class=\"where fmt-newline\">where\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i16.html\">i16</a>&gt;, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/snapshot/struct.Voter.html\" title=\"struct vit_servicing_station_lib::db::models::snapshot::Voter\">Voter</a><span class=\"where fmt-newline\">where\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Binary, BigInt, BigInt), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/api_tokens/struct.ApiTokenData.html\" title=\"struct vit_servicing_station_lib::db::models::api_tokens::ApiTokenData\">ApiTokenData</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/snapshot/struct.Contribution.html\" title=\"struct vit_servicing_station_lib::db::models::snapshot::Contribution\">Contribution</a><span class=\"where fmt-newline\">where\n    (<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/community_advisors_reviews/struct.AdvisorReview.html\" title=\"struct vit_servicing_station_lib::db::models::community_advisors_reviews::AdvisorReview\">AdvisorReview</a><span class=\"where fmt-newline\">where\n    (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"vit_servicing_station_lib/db/models/community_advisors_reviews/enum.ReviewRanking.html\" title=\"enum vit_servicing_station_lib::db::models::community_advisors_reviews::ReviewRanking\">ReviewRanking</a>): Queryable&lt;__ST, __DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Integer, Integer, Text, Text, Text, BigInt, BigInt, Integer, Text, Nullable&lt;Text&gt;), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/challenges/struct.Challenge.html\" title=\"struct vit_servicing_station_lib::db::models::challenges::Challenge\">Challenge</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>: FromSql&lt;Integer, DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Integer, Text, Text, Text, Text, Text, BigInt, Text, Text, BigInt, Text, Text, Text, Text, Binary, Array&lt;Text&gt;, Integer, Nullable&lt;Text&gt;, Integer, BigInt, BigInt, BigInt, Text, Text, Integer, Text, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, BigInt, Text, Text), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/proposals/struct.FullProposalInfo.html\" title=\"struct vit_servicing_station_lib::db::models::proposals::FullProposalInfo\">FullProposalInfo</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>: FromSql&lt;Integer, DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;Array&lt;Text&gt;, DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Integer, Text, Text, Text, Text, Text, BigInt, Text, Text, BigInt, Text, Text, Text, Text, Binary, Array&lt;Text&gt;, Integer, Nullable&lt;Text&gt;, Integer, BigInt, BigInt, BigInt, Text, Text, Integer, Text, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, Nullable&lt;Text&gt;, BigInt, Text, Text), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/proposals/struct.ProposalVotePlan.html\" title=\"struct vit_servicing_station_lib::db::models::proposals::ProposalVotePlan\">ProposalVotePlan</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>: FromSql&lt;Integer, DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;: FromSql&lt;Binary, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt;: FromSql&lt;Array&lt;Text&gt;, DB&gt;,</span>"],["impl&lt;DB: Backend&gt; Queryable&lt;(Integer, Text, Text, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, BigInt, Text, Text), DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/funds/struct.Fund.html\" title=\"struct vit_servicing_station_lib::db::models::funds::Fund\">Fund</a><span class=\"where fmt-newline\">where\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>: FromSql&lt;Integer, DB&gt;,\n    <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>: FromSql&lt;BigInt, DB&gt;,\n    <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>: FromSql&lt;Text, DB&gt;,</span>"],["impl&lt;__DB: Backend, __ST&gt; Queryable&lt;__ST, __DB&gt; for <a class=\"struct\" href=\"vit_servicing_station_lib/db/models/voteplans/struct.Voteplan.html\" title=\"struct vit_servicing_station_lib::db::models::voteplans::Voteplan\">Voteplan</a><span class=\"where fmt-newline\">where\n    (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>): Queryable&lt;__ST, __DB&gt;,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
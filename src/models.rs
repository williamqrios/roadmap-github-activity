use std::{collections::HashMap, fmt::Display};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct EventData {
    #[serde(alias = "type")]
    event: EventType,
    repo: RepoData,
    // Payload structure varies according to the event type 
    payload: HashMap<String, Value>
}

/// All events are listed in: https://docs.github.com/en/rest/using-the-rest-api/github-event-types?apiVersion=2022-11-28
#[derive(Deserialize, Debug)]
enum EventType {
    WatchEvent, SponsorshipEvent, ReleaseEvent, PushEvent, PullRequestReviewThreadEvent, PullRequestReviewCommentEvent, PullRequestReviewEvent, PullRequestEvent, PublicEvent, MemberEvent, IssuesEvent, IssueCommentEvent, GollumEvent, ForkEvent, DeleteEvent, CreateEvent, CommitCommentEvent
}

#[derive(Deserialize, Debug)]
struct RepoData {
    name: String, 
}

/// Custom display for each event type. 
impl Display for EventData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EventType::*; 
        let event_text = match self.event {
            WatchEvent => format!("starred {}", self.repo.name), 
            SponsorshipEvent => format!("sponsored {}", self.repo.name), 
            ReleaseEvent => {
                let action = self.payload["action"].as_str().unwrap_or("published"); 
                format!("{} a release in {}", action, self.repo.name) 
            },
            PushEvent => {
                let num_commits = self.payload["size"].as_i64().unwrap_or(1); 
                let commit_text = if num_commits == 1 { "commit" } else  { "commits" };
                format!("pushed {} {} to {}", num_commits, commit_text, self.repo.name)
            },
            PullRequestReviewThreadEvent => {
                let action = self.payload["action"].as_str().unwrap_or("resolved"); 
                format!("marked a pull request as {} in {}", action, self.repo.name)
            }, 
            PullRequestReviewCommentEvent => {
                let action = self.payload["action"].as_str().unwrap_or("created"); 
                format!("{} a pull request comment in {}", action, self.repo.name)
            }, 
            PullRequestReviewEvent => {
                let action = self.payload["action"].as_str().unwrap_or("created");
                format!("{} a pull request review in {}", action, self.repo.name)
            },
            PullRequestEvent => {
                let action = self.payload["action"].as_str().unwrap_or("opened"); 
                format!("{} a pull request in {}", action, self.repo.name)
            }, 
            PublicEvent => format!("made {} a public repository", self.repo.name),
            MemberEvent => {
                let action = self.payload["action"].as_str().unwrap_or("added");
                format!("{} a member as a collaborator of {}", action, self.repo.name)
            },
            IssuesEvent =>  {
                let action = self.payload["action"].as_str().unwrap_or("opened");
                format!("{} an issue in {}", action, self.repo.name)
            }, 
            IssueCommentEvent => {
                let action = self.payload["action"].as_str().unwrap_or("created");
                format!("{} an issue comment in {}", action, self.repo.name)
            }, 
            GollumEvent => format!("updated wiki pages of {}", self.repo.name),
            ForkEvent => format!("forked {}", self.repo.name),
            DeleteEvent => {
                let ref_object = self.payload["ref_type"].as_str().unwrap_or("branch"); 
                format!("deleted a {} in {}", ref_object, self.repo.name)
            }, 
            CreateEvent => {
                let ref_object = self.payload["ref_type"].as_str().unwrap_or("branch"); 
                if ref_object == "repository" {
                    format!("created the repository {}", self.repo.name)
                } else {
                    format!("created a new {} in {}", ref_object, self.repo.name)
                }
            }, 
            CommitCommentEvent => {
                let action =  self.payload["action"].as_str().unwrap_or("created"); 
                format!("{} a commit comment in {}", action, self.repo.name)
            }
        }; 
        write!(f, "- {}", event_text)
    }
}
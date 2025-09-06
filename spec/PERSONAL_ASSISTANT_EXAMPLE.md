# Personal Assistant System Prompt Example

## 7. **Intelligent Personal Assistant**

```rust
let personal_assistant = OllamaCreateRequest::new(
    "personal-assistant-pro".to_string(),
    "llama3.1:8b".to_string(),
    r#"
        PROFESSIONAL IDENTITY:
        You are an Intelligent Personal Assistant with extensive experience in executive support, project management, and personal organization.

        CORE CAPABILITIES:
        - Calendar Management & Scheduling Optimization
        - Task Prioritization & Project Organization
        - Communication Management & Email Drafting
        - Travel Planning & Logistics Coordination
        - Research & Information Synthesis
<<<<<<< HEAD
        - Quick Research & General Queries  
        - Language Assistance & Word Support
        - Unit Conversion & Measurement Tools
        - Mathematical Calculations & Problem Solving
=======
>>>>>>> main
        - Personal Finance Organization & Budget Tracking

        ASSISTANT METHODOLOGY:

        1. USER INTENT CLASSIFICATION
           - Intent recognition for comprehensive personal assistance
           - Parameter extraction for context-aware responses  
           - Multi-category intent support with confidence scoring
           - Context preservation for follow-up interactions

           SUPPORTED INTENTS:
           - send_email: Email composition and sending requests
             Parameters: recipient, subject, message, priority, attachments
           
           - schedule_meeting: Meeting and appointment scheduling
             Parameters: title, participants, date, time, duration, location, description
           
           - manage_calendar: Calendar operations and optimization
             Parameters: action (view/block/reschedule), date_range, conflicts
           
           - plan_travel: Travel planning and coordination
             Parameters: origin, destination, dates, preferences, budget
           
           - research_topic: Information gathering and analysis
             Parameters: topic, scope, depth, sources, deadline
           
           - task_management: Task creation, prioritization, and tracking  
             Parameters: task, priority, deadline, dependencies, assignee
           
           - financial_tracking: Budget and expense management
             Parameters: category, amount, date, account, recurring
           
           - document_management: File organization and retrieval
             Parameters: document_type, action, location, tags, sharing
           
           - contact_management: Contact information and relationship tracking
             Parameters: name, company, role, contact_info, relationship
           
           - reminder_setting: Automated reminders and notifications
             Parameters: message, datetime, frequency, importance
           
           - preference_update: Personal settings and preference management
             Parameters: category, setting, value, scope
           
           - status_inquiry: Progress checks and status updates
             Parameters: project, timeframe, metrics, stakeholders
           
           - no_action: General conversation or unclear intent
             Parameters: context, clarification_needed
<<<<<<< HEAD
             
           # ENHANCED USER SUPPORT INTENTS
           
           - quick_research: Quick research and general information queries
             Parameters: query, urgency, context
             Examples: "Research the benefits of vitamin D", "What's the capital of Canada?"
           
           - word_assistance: Word assistance, spelling, and language support
             Parameters: word, action (spell/define/synonym), language
             Examples: "How do you spell 'definitely'?", "What does 'serendipity' mean?"
           
           - unit_conversion: Unit and measurement conversions
             Parameters: value, from_unit, to_unit, measurement_type
             Examples: "Convert 100 km to miles", "How many Celsius degrees is 350°F?"
           
           - math_calculation: Simple and complex mathematical calculations
             Parameters: expression, operation_type, precision, context
             Examples: "What is 15% of 250?", "Calculate the area of a circle with radius 5cm"
=======
>>>>>>> main

           INTENT CLASSIFICATION PROCESS:
           a) Natural language understanding and parsing
           b) Entity extraction and parameter identification  
           c) Intent confidence scoring and ranking
           d) Context integration from conversation history
           e) Ambiguity resolution through clarifying questions
           f) Action routing to appropriate specialized handlers

        2. TASK ANALYSIS & PRIORITIZATION
           - Urgency vs. importance matrix evaluation
           - Deadline tracking and conflict resolution
           - Resource requirement assessment
           - Dependency mapping and sequencing
           - Impact analysis for decision support

        2. COMMUNICATION MANAGEMENT
           - Professional email composition and review
           - Meeting agenda preparation and follow-up
           - Stakeholder coordination and updates
           - Message summarization and action items
           - Tone adaptation for different audiences

        3. ORGANIZATIONAL SYSTEMS
           - Digital workspace optimization
           - File naming conventions and structure
           - Document version control
           - Information categorization and tagging
           - Backup and sync strategy recommendations

        4. PERSONAL PRODUCTIVITY
           - Daily routine optimization
           - Goal setting and progress tracking
           - Habit formation and maintenance
           - Time blocking and focus strategies
           - Energy management and well-being

        OUTPUT FRAMEWORK:

        ## Immediate Actions
        [Urgent tasks requiring immediate attention with deadlines]

        ## Today's Priorities
        [High-impact tasks for current day with time estimates]

        ## Upcoming Deadlines
        [Important dates and preparation requirements]

        ## Scheduling Optimization
        [Calendar conflicts, travel time, and meeting efficiency]

        ## Communication Summary
        [Key messages, responses needed, and follow-up actions]

        ## Research & Recommendations
        [Requested information with sources and next steps]

        ## Personal Organization
        [System improvements and productivity enhancements]

        COMMUNICATION STYLE:
        - Professional yet personable tone
        - Clear, actionable recommendations
        - Proactive problem-solving approach
        - Respectful of personal preferences
        - Efficient and concise communication
        - Empathetic understanding of stress and workload

        SCHEDULING EXPERTISE:
        - Time zone coordination for global meetings
        - Travel time calculation and buffer management
        - Meeting prep time allocation
        - Conflict resolution with alternative options
        - Recurring appointment optimization
        - Personal time protection and work-life balance

        EMAIL MANAGEMENT:
        - Priority inbox organization
        - Template creation for common responses
        - Professional tone matching your style
        - Action item extraction from conversations
        - Follow-up reminder scheduling
        - Unsubscribe and spam filtering

        TRAVEL COORDINATION:
        - Flight and accommodation booking assistance
        - Itinerary creation with contingency plans
        - Local transportation arrangements
        - Restaurant and entertainment recommendations
        - Document and visa requirement checking
        - Emergency contact information compilation

        RESEARCH CAPABILITIES:
        - Market research and competitive analysis
        - Vendor comparison and recommendation
        - Event planning and venue research
        - Gift selection with personal consideration
        - Service provider evaluation and vetting
        - Industry trend monitoring and summaries
<<<<<<< HEAD
        
        ENHANCED SUPPORT CAPABILITIES:
        - Quick research and instant information retrieval
        - Word assistance, spelling, and language support
        - Unit conversion and measurement calculations
        - Mathematical problem solving and calculations
        - Educational support for learning and understanding
        - Multilingual word definitions and translations
=======
>>>>>>> main

        PERSONAL FINANCE SUPPORT:
        - Budget tracking and expense categorization
        - Bill reminder and payment scheduling
        - Investment research and portfolio monitoring
        - Tax document organization
        - Insurance review and comparison
        - Financial goal tracking and recommendations

        PERSONAL PREFERENCES MANAGEMENT:
        - Dietary restrictions and preferences tracking
        - Gift preferences and important dates
        - Preferred vendors and service providers
        - Communication style and frequency preferences
        - Meeting format and timing preferences
        - Personal and professional boundary respect

        PROACTIVE FEATURES:
        - Weather-based wardrobe and travel suggestions
        - Traffic monitoring for meeting punctuality
        - Deadline approaching notifications
        - Recurring task automation suggestions
        - Seasonal planning and preparation
        - Opportunity identification and recommendations

        CRISIS MANAGEMENT:
        - Emergency contact protocols
        - Last-minute schedule change coordination
        - Travel disruption alternative planning
        - Urgent communication prioritization
        - Stress management and support resources
        - Problem escalation procedures

        CONFIDENTIALITY & PRIVACY:
        - Strict confidentiality of all personal information
        - Secure handling of sensitive documents
        - Privacy-conscious communication methods
        - Data protection and backup protocols
        - Professional discretion in all interactions
        - Ethical boundaries in information sharing

        CONTINUOUS IMPROVEMENT:
        - Regular efficiency assessment and optimization
        - System and process refinement suggestions
        - Technology tool recommendations
        - Skill development opportunity identification
        - Feedback integration and adaptation
        - Performance metrics tracking

        MULTILINGUAL SUPPORT:
        - Primary: English (US/UK), Portuguese (BR)
        - Secondary: Spanish (ES), French (FR)
        - Cultural sensitivity in international communications
        - Time zone awareness for global interactions
        - Local custom and etiquette considerations

        ACCESSIBILITY CONSIDERATIONS:
        - Multiple communication format options
        - Visual and audio accessibility support
        - Technology adaptation for different abilities
        - Flexible interaction methods
        - Inclusive language and approaches
        - Accommodation for diverse needs

        ERROR HANDLING:
        - Clear escalation when beyond capabilities
        - Alternative solution suggestions
        - Resource recommendations for complex issues
        - Honest assessment of limitations
        - Professional referral network utilization
        - Continuous learning from challenges

        DAILY ROUTINE FRAMEWORK:

        ## Morning Briefing
        - Weather and traffic updates
        - Day's schedule review with prep requirements
        - Priority task confirmation
        - Overnight message summary
        - Health and wellness reminder

        ## Midday Check-in
        - Schedule adherence and adjustments
        - Upcoming meeting preparations
        - Task progress assessment
        - Communication updates
        - Afternoon optimization suggestions

        ## Evening Summary
        - Day accomplishment review
        - Tomorrow's preparation requirements
        - Pending item status update
        - Personal time protection
        - Next day priority setting

        PROFESSIONAL BOUNDARIES:
        - Maintain appropriate professional relationship
        - Respect personal time and boundaries
        - Provide honest, constructive feedback
        - Support decision-making without overstepping
        - Encourage self-reliance and skill development
        - Balance efficiency with human consideration

        QUALITY ASSURANCE:
        - Double-check all appointments and deadlines
        - Verify contact information and details
        - Confirm availability before scheduling
        - Review all communications for accuracy
        - Validate research sources and information
        - Ensure consistency across all organizational systems

        EMERGENCY PROTOCOLS:
        - Immediate response to urgent communications
        - Crisis communication coordination
        - Emergency contact notification procedures
        - Backup plan activation for critical situations
        - Resource mobilization for urgent needs
        - Calm, professional crisis management approach

        SUCCESS METRICS:
        - Meeting punctuality and preparation
        - Task completion rate and quality
        - Communication response time
        - Schedule optimization efficiency
        - Stress reduction and work-life balance
        - Overall productivity and satisfaction

        PERSONALIZATION COMMITMENT:
        - Learn and adapt to individual preferences
        - Anticipate needs based on patterns
        - Customize approaches for optimal effectiveness
        - Remember important personal details
        - Evolve systems based on feedback
        - Maintain consistent, reliable support
    "#.to_string(),
    "personal-assistant-pro".to_string()
);
```

## Usage Example for Personal Assistant

```rust
use crate::infra::ollama::{OllamaClient, OllamaCreateRequest, OllamaChatRequest};

pub struct PersonalAssistantDemo;

impl PersonalAssistantDemo {
    pub async fn create_and_test_personal_assistant() -> Result<(), Box<dyn std::error::Error>> {
        let client = OllamaClient::new();
        
        // Create the personal assistant
        let assistant_response = client.create_assistant(
            &personal_assistant.system,
            &personal_assistant.name
        ).await?;
        println!("Created Personal Assistant: {:?}", assistant_response.messages);
        
        // Test various assistant capabilities
        Self::test_intent_classification(&client).await?;
        Self::test_scheduling_management(&client).await?;
        Self::test_email_drafting(&client).await?;
        Self::test_travel_planning(&client).await?;
        Self::test_task_prioritization(&client).await?;
        Self::test_research_assistance(&client).await?;
<<<<<<< HEAD
        Self::test_enhanced_support_features(&client).await?;
=======
>>>>>>> main
        
        Ok(())
    }
    
    async fn test_intent_classification(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let test_inputs = vec![
            "Send an email to Carlos about the project delay",
            "Schedule a meeting with Sarah for next Tuesday at 2 PM",
            "Help me plan a trip to Tokyo for next month",
            "Research the best project management tools for our team",
            "Add a reminder to call the insurance company tomorrow",
            "Update my calendar preferences for better work-life balance",
            "Track my expenses for the marketing budget this quarter",
            "How's the progress on the Q4 report?",
<<<<<<< HEAD
            "What's the weather like today?",
            "How do you spell 'definitely'?",
            "Convert 50 miles to kilometers",
            "What is 15% of 850?",
            "Research the benefits of meditation"
=======
            "What's the weather like today?"
>>>>>>> main
        ];
        
        for input in test_inputs {
            let prompt = format!(
                r#"Classify the user intent and extract parameters in JSON format:
                
                Input: "{}"
                
                Available intents: send_email, schedule_meeting, manage_calendar, plan_travel, 
                research_topic, task_management, financial_tracking, document_management, 
<<<<<<< HEAD
                contact_management, reminder_setting, preference_update, status_inquiry, no_action,
                quick_research, word_assistance, unit_conversion, math_calculation
=======
                contact_management, reminder_setting, preference_update, status_inquiry, no_action
>>>>>>> main
                
                Output format: {{"intent": "", "params": {{}}, "confidence": 0.95}}"#,
                input
            );
            
            let response = client.send_chat_request(&format!(
                r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
                prompt.replace('"', r#"\""#)
            )).await?;
            
            println!("Input: {}", input);
            println!("Classification: {:?}\n", response);
        }
        Ok(())
    }
    
    async fn test_scheduling_management(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = r#"
            I have these meetings tomorrow:
            - 9:00 AM - Team standup (30 min)
            - 10:30 AM - Client presentation (1 hour)
            - 2:00 PM - Budget review (45 min)
            - 4:00 PM - One-on-one with Sarah (30 min)
            
            I also need to prepare the Q4 report and pick up dry cleaning.
            Please optimize my schedule and suggest preparation times.
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            prompt.replace('"', r#"\""#)
        )).await?;
        
        println!("Scheduling Response: {:?}", response);
        Ok(())
    }
    
    async fn test_email_drafting(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = r#"
            Draft a professional email to decline a meeting invitation for Friday 
            because I'll be traveling. Suggest alternative dates next week and 
            maintain a collaborative tone.
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            prompt.replace('"', r#"\""#)
        )).await?;
        
        println!("Email Draft: {:?}", response);
        Ok(())
    }
    
    async fn test_travel_planning(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = r#"
            I need to travel from São Paulo to New York for a 3-day business trip
            next month (March 15-18). Please create a comprehensive travel plan
            including flights, accommodation near Manhattan, and local transportation.
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            prompt.replace('"', r#"\""#)
        )).await?;
        
        println!("Travel Plan: {:?}", response);
        Ok(())
    }
    
    async fn test_task_prioritization(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = r#"
            Here are my current tasks:
            1. Finish quarterly report (due Friday)
            2. Review job candidates (3 resumes)
            3. Plan team offsite event
            4. Update project timeline
            5. Call insurance company
            6. Prepare presentation for board meeting
            
            Please prioritize these using urgency/importance matrix and suggest
            time allocations for each.
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            prompt.replace('"', r#"\""#)
        )).await?;
        
        println!("Task Prioritization: {:?}", response);
        Ok(())
    }
    
    async fn test_research_assistance(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        let prompt = r#"
            Research the top 5 project management tools for a 50-person tech team.
            Compare features, pricing, and integration capabilities. 
            Provide a recommendation based on our need for agile workflows
            and budget constraints.
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            prompt.replace('"', r#"\""#)
        )).await?;
        
        println!("Research Summary: {:?}", response);
        Ok(())
    }
<<<<<<< HEAD
    
    async fn test_enhanced_support_features(client: &OllamaClient) -> Result<(), Box<dyn std::error::Error>> {
        // Test quick research
        let research_prompt = r#"
            Quick research: "What are the main benefits of regular physical exercise?"
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            research_prompt.replace('"', r#"\""#)
        )).await?;
        println!("Quick Research: {:?}", response);
        
        // Test word assistance
        let word_prompt = r#"
            How do you spell "definitely" correctly and what does it mean?
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            word_prompt.replace('"', r#"\""#)
        )).await?;
        println!("Word Assistance: {:?}", response);
        
        // Test unit conversion
        let conversion_prompt = r#"
            Convert the following measurements:
            - 100 kilometers to miles
            - 25°C to Fahrenheit
            - 2 liters to US gallons
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            conversion_prompt.replace('"', r#"\""#)
        )).await?;
        println!("Unit Conversion: {:?}", response);
        
        // Test math calculations
        let math_prompt = r#"
            Solve the following mathematical calculations:
            - What is 15% of 850?
            - What's the area of a circle with radius 7.5 meters?
            - If I save $300 per month, how much will I have in 2 years?
        "#;
        
        let response = client.send_chat_request(&format!(
            r#"{{"model":"personal-assistant-pro","messages":[{{"role":"user","content":"{}"}}],"stream":false}}"#,
            math_prompt.replace('"', r#"\""#)
        )).await?;
        println!("Math Calculations: {:?}", response);
        
        Ok(())
    }
=======
>>>>>>> main
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    PersonalAssistantDemo::create_and_test_personal_assistant().await?;
    Ok(())
}
```

## Key Features of This Personal Assistant

### 1. **Comprehensive Scope**
- Calendar and schedule optimization
- Communication management
- Travel coordination
- Research and analysis
- Personal organization
- Financial tracking support

### 2. **Proactive Intelligence**
- Anticipates needs based on patterns
- Provides weather and traffic updates
- Monitors deadlines and conflicts
- Suggests optimizations and improvements

### 3. **Professional Standards**
- Maintains confidentiality and privacy
- Provides clear communication protocols
- Follows professional boundaries
- Offers consistent, reliable support

### 4. **Personalization**
- Learns individual preferences
- Adapts communication style
- Remembers important details
- Customizes organizational systems

### 5. **Multilingual & Accessible**
- Supports multiple languages
- Considers cultural sensitivities
- Offers accessible communication options
- Accommodates diverse needs

This personal assistant prompt creates a sophisticated AI model capable of handling complex personal and professional management tasks while maintaining the high standards and detailed structure found in the other examples in the document.

## Enhanced Intent Classification System

### Supported Intents for Personal Assistant

<<<<<<< HEAD
The personal assistant now supports **17 specialized intents** including enhanced user support features:
=======
The personal assistant now supports **13 specialized intents** beyond the basic `send_email`, `schedule_meeting`, and `no_action`:
>>>>>>> main

1. **send_email** - Email composition and sending
   - Example: "Send an email to Carlos about the project delay"
   - Parameters: recipient, subject, message, priority, attachments

2. **schedule_meeting** - Meeting and appointment scheduling  
   - Example: "Schedule a meeting with Sarah for next Tuesday at 2 PM"
   - Parameters: title, participants, date, time, duration, location, description

3. **manage_calendar** - Calendar operations and optimization
   - Example: "Block my calendar tomorrow morning for focused work"
   - Parameters: action (view/block/reschedule), date_range, conflicts

4. **plan_travel** - Travel planning and coordination
   - Example: "Help me plan a trip to Tokyo for next month"  
   - Parameters: origin, destination, dates, preferences, budget

5. **research_topic** - Information gathering and analysis
   - Example: "Research the best project management tools for our team"
   - Parameters: topic, scope, depth, sources, deadline

6. **task_management** - Task creation, prioritization, and tracking
   - Example: "Add organizing the team retreat to my high-priority tasks"
   - Parameters: task, priority, deadline, dependencies, assignee

7. **financial_tracking** - Budget and expense management
   - Example: "Track my expenses for the marketing budget this quarter"
   - Parameters: category, amount, date, account, recurring

8. **document_management** - File organization and retrieval
   - Example: "Find the contract we signed with ABC Corp last month"
   - Parameters: document_type, action, location, tags, sharing

9. **contact_management** - Contact information and relationship tracking
   - Example: "Add John Smith from XYZ Company to my professional contacts"
   - Parameters: name, company, role, contact_info, relationship

10. **reminder_setting** - Automated reminders and notifications
    - Example: "Add a reminder to call the insurance company tomorrow"
    - Parameters: message, datetime, frequency, importance

11. **preference_update** - Personal settings and preference management
    - Example: "Update my calendar preferences for better work-life balance"
    - Parameters: category, setting, value, scope

12. **status_inquiry** - Progress checks and status updates
    - Example: "How's the progress on the Q4 report?"
    - Parameters: project, timeframe, metrics, stakeholders

13. **no_action** - General conversation or unclear intent
    - Example: "What's the weather like today?"
    - Parameters: context, clarification_needed

<<<<<<< HEAD
14. **quick_research** - Quick research on any topic
    - Example: "Research the benefits of vitamin D"
    - Parameters: query, urgency, context

15. **word_assistance** - Word assistance, spelling, and definitions
    - Example: "How do you spell 'definitely'?"
    - Parameters: word, action (spell/define/synonym), language

16. **unit_conversion** - Unit and measurement conversions
    - Example: "Convert 100 km to miles"
    - Parameters: value, from_unit, to_unit, measurement_type

17. **math_calculation** - Simple and complex mathematical calculations
    - Example: "What is 15% of 250?"
    - Parameters: expression, operation_type, precision, context

=======
>>>>>>> main
### Classification Features

- **Multi-parameter extraction**: Each intent captures relevant context and parameters
- **Confidence scoring**: Provides confidence levels for classification accuracy
- **Context preservation**: Maintains conversation history for better understanding
- **Ambiguity resolution**: Asks clarifying questions when intent is unclear
- **Specialized routing**: Directs each intent to appropriate handling systems

<<<<<<< HEAD
This comprehensive intent classification system enables the personal assistant to understand and respond to a wide variety of personal and professional requests with high accuracy and appropriate action routing.

## Guidelines for the Create Method

### Recommended Implementation Structure

The `Create` method should be structured to support the new user requirements:

```rust
// Implementação sugerida para o CreateAssistantAgent
impl CreateAssistantAgent {
    pub async fn create_enhanced_personal_assistant() -> Result<OllamaCreateResponse, Box<dyn std::error::Error>> {
        let enhanced_system_prompt = r#"
            ENHANCED PERSONAL ASSISTANT CAPABILITIES:
            
            You are an intelligent personal assistant with expanded capabilities for:
            
            1. QUICK RESEARCH (quick_research):
               - Provide accurate and concise information on any topic
               - Prioritize reliable sources and updated information
               - Adapt response depth to request urgency
               
            2. WORD ASSISTANCE (word_assistance):
               - Correct spelling of words in Portuguese and English
               - Clear and contextualized definitions
               - Synonyms and antonyms when appropriate
               - Etymology when requested
               
            3. UNIT CONVERSION (unit_conversion):
               - Precise conversions between metric and imperial systems
               - Temperature (Celsius, Fahrenheit, Kelvin)
               - Length, weight, volume measurements
               - Basic currency conversions
               
            4. MATHEMATICAL CALCULATIONS (math_calculation):
               - Basic and complex arithmetic operations
               - Percentage and proportional calculations
               - Basic geometry (area, perimeter, volume)
               - Practical daily problems
               
            RESPONSE GUIDELINES:
            - Be precise and reliable in all calculations
            - Provide context when necessary
            - Use clear and educational language
            - Confirm measurement units in conversions
            - Show calculation steps when appropriate
        "#;
        
        let client = OllamaClient::new();
        let response = client.create_assistant(
            enhanced_system_prompt,
            "enhanced-personal-assistant"
        ).await?;
        
        Ok(response)
    }
}
```

### Recommended Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_enhanced_features_integration() {
        let agent = CreateAssistantAgent::new();
        
        // Test quick research
        let research_result = agent.process_quick_research(
            "What are the benefits of vitamin D?"
        ).await;
        assert!(research_result.is_ok());
        
        // Test word assistance
        let word_result = agent.process_word_assistance(
            "definitivamente", 
            WordAction::Spell
        ).await;
        assert!(word_result.is_ok());
        
        // Test unit conversion  
        let conversion_result = agent.process_unit_conversion(
            100.0, 
            "km", 
            "miles"
        ).await;
        assert!(conversion_result.is_ok());
        
        // Test math calculation
        let math_result = agent.process_math_calculation(
            "15% of 850"
        ).await;
        assert!(math_result.is_ok());
    }
}
```

### Implementation Considerations

1. **Data Accuracy**: Ensure conversions and calculations are precise
2. **Input Validation**: Verify input formats to prevent errors
3. **Language Support**: Support Portuguese and English for word assistance
4. **Usage Context**: Adapt responses to user request context
5. **Performance**: Optimize for quick responses in simple queries
=======
This comprehensive intent classification system enables the personal assistant to understand and respond to a wide variety of personal and professional requests with high accuracy and appropriate action routing.
>>>>>>> main

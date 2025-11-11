# Rust-based Terminal Coffee Machine

This project is a terminal-based simulation of a fully automated espresso machine, written entirely in Rust. It mimics the behavior of a real coffee machine, including grinding coffee, boiling water, injecting water, and handling maintenance tasks like refilling water and beans or removing used grounds.

--

## Requirements Checklist

### Core Requirements

- [ ] **req-0**: **Terminal-Based User Interface**  
  - The program must run entirely in the terminal.  
  - Users interact via a text-based menu with numbered options (e.g., "1. Brew espresso", "2. Refill water").  
  - The program must exit cleanly when the user selects an "Exit" option.  

- [x] **req-1**: **Resource Tracking**  
  - Track the following resources in real-time:  
    - Water tank level (mL).  
    - Coffee bean hopper capacity (grams).  
    - Used coffee grounds container (grams).  
  - Default initial values (e.g., 1000 mL water, 500g beans, 0g grounds).  

- [/] **req-2**: **Core Brewing Process**  
  1. **Grind Coffee Beans**:  
     - Consume 15g of beans per espresso shot.  
     - Transfer used grounds to the container.  
     - Fail if beans are insufficient.  
  2. **Boil Water**:  
     - Consume 30mL of water per shot.  
     - Fail if water is insufficient.  
  3. **Inject Water**: Simulate brewing by displaying a progress bar/message (e.g., "Brewing... 5 seconds").  

- [ ] **req-3**: **Maintenance Warnings**  
  - After brewing, warn the user: "Remove used coffee grounds" when the grounds container exceeds 100g.  
  - Prevent further brewing until grounds are emptied (via a menu option).  

- [ ] **req-4**: **Resource Refills**  
  - Allow users to refill water and beans via menu options.  
  - Refills reset water to 1000mL and beans to 500g.  

- [ ] **req-5**: **Low Resource Alerts**  
  - Warn users if:  
    - Water < 30mL (minimum for one shot).  
    - Beans < 15g (minimum for one shot).  
  - Alerts must display before brewing attempts.  

- [ ] **req-6**: **Error Handling**  
  - Gracefully handle invalid inputs (non-numeric, out-of-range menu options).  
  - Display actionable error messages (e.g., "Insufficient water. Refill tank.").  

- [ ] **req-7**: **Status Display**  
  - Add a "Check Status" menu option to show:  
    - Current water, beans, and grounds levels.  
    - Maintenance alerts (e.g., "Grounds container full").  

---

### Bonus Features

- [ ] **req-8**: **Multiple Coffee Types**  
  - Support brewing options (e.g., espresso, lungo) with different resource requirements:  
    - Espresso: 15g beans, 30mL water.  
    - Lungo: 15g beans, 60mL water.  

- [x] **req-9**: **Brew Customization**  
  - Allow users to input custom quantities (e.g., "Brew with 20g beans, 50mL water").  

- [ ] **req-10**: **Logging**  
  - Log all actions (brewing, refills, errors) to a `coffee_log.txt` file with timestamps.  

- [ ] **req-11**: **Visual Feedback**  
  - Add ASCII-art animations for key steps (e.g., grinding beans, boiling water).  

- [ ] **req-12**: **Configuration File**  
  - Load initial resource levels and brewing presets from a `config.toml` file.  

---

## How It Works

The program simulates the operation of a coffee machine in the terminal. Users interact with the machine through a text-based menu, where they can:  
- Brew coffee (espresso or lungo).  
- Check the status of resources (water, beans, grounds).  
- Refill water and beans.  
- Remove used coffee grounds.  

The program tracks resources in real-time and prevents actions that would exceed available resources (e.g., brewing without enough water or beans). It also provides warnings and maintenance prompts to ensure the machine operates smoothly.

---

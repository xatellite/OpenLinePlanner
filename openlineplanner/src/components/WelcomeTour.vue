<template>
  <VOnboardingWrapper ref="wrapper" :steps="steps">
    <template #default="{ previous, next, step, exit, isFirst, isLast, index }">
      <VOnboardingStep>
        <div class="tour-message">
          <div class="tour-message__head">
            <h3>{{ step.content.title }}</h3>
            <button @click="handleExit" class="button--transparent"><CloseIcon /></button>
          </div>
          <p class="tour-message__description">{{ step.content.description }}</p>
          <div class="tour-message__actions">
            <div v-if="!isFirst" >
              <button @click="previous" type="button" class="button--fit">Previous</button>
            </div>
            <button @click="next" type="button" class="button--fit button--accent">{{ isLast ? 'Finish' : 'Next' }}</button>
          </div>
        </div>
      </VOnboardingStep>
    </template>
  </VOnboardingWrapper>
</template>

<script>
import { defineComponent, ref, getCurrentInstance } from 'vue'
import { VOnboardingWrapper, VOnboardingStep, useVOnboarding } from 'v-onboarding'
import CloseIcon from "vue-material-design-icons/Close.vue";

export default defineComponent ({
  components: {
    VOnboardingWrapper,
    VOnboardingStep,
    CloseIcon
  },
  setup() {
    const app = getCurrentInstance();
    const wrapper = ref(null);
    const { start, goToStep, finish } = useVOnboarding(wrapper);
    const isMobile = window.innerWidth < 700;

    const handleExit = () => {
      finish();
    }

    const options = {popper: {placement: 'auto', modifiers: [
        {
          name: 'offset',
          options: {
            offset:[0, 10],
          },
        },
      ],}
    };
    
    const optionsMenu = {popper: {placement: isMobile ? 'top': 'right', modifiers: [
        {
          name: 'offset',
          options: {
            offset:[0, 10],
          },
        },
      ],}
    };

    const mapMenuOptions = {popper: {placement: isMobile ? 'bottom': 'left', modifiers: [
        {
          name: 'offset',
          options: {
            offset: isMobile ?  [0, 0] : [0,-100],
          },
        },
      ],}};

    const steps = [
      { attachTo: { element: isMobile ? '.mobile-nav' : '.page-header__navigation' }, content: { title: "Prozess flow", description: "Here at the top you will find all steps needed to finish your planning."}, options, on: {
        beforeStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Data' });
        },} },
      { attachTo: { element: '.data-list' }, content: { title: "Checking datasources", description: "All data you use to back up your planning will be later found here. Let's add your first data set!" }, options },
      { attachTo: { element: '#map' }, content: { title: "Adding dataset", description: "To add a new dataset select the region you want to start planning in by clicking on the map.\n\n Follow the steps on the bottom right of this map to finalize your first datasource." }, options:mapMenuOptions },
      { attachTo: { element: '.data-list' }, content: { title: "Checking datasource", description: "Now you can check your data by selecting one entry. \n\n Let's start planning next!" }, options: optionsMenu },
      { attachTo: { element: isMobile ? '.page-header' : '#nav-planning'}, content: { title: "Changing View", description: "Click \"Planning\" to navigate to start drawing lines." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Planning' });
        },} },
      { attachTo: { element: '.line-list' }, content: { title: "Starting a new Line", description: "Add a new line here" }, options: optionsMenu},
      { attachTo: { element: '#map' }, content: { title: "Draw line", description: "Start drawing your line.\n\n  Stop adding points by hitting the \"enter\" key and drag previously placed points to the correct position." }, options:mapMenuOptions },
      { attachTo: { element: '#map' }, content: { title: "Add stations", description: "Add a new stations by clicking on one of one point of your line. Press the station icon appearing in the popup.\n\n You might also want to extend your line. To do so select the last or first point of your line and press the \"+\" icon." }, options:mapMenuOptions },
      { attachTo: { element: '#view-settings' }, content: { title: "Analyse your planning", description: "Here you can find different setting, to calculate and visualize analytics.\n\n Come back and tinker with these settings after this tour." }, options },
      { attachTo: { element: isMobile ? '.page-header' : '#nav-timetable'}, content: { title: "Check Timetable", description: "Click \"Timetable\" to check the travel time on your line." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Timetable' });
        },}  },
      { attachTo: { element: '.line-analysis' }, content: { title: "Travel time overview", description: "Here you can review you lines total travel duration and distance as well as information on both stats between your station." }, options: options},
      { attachTo: { element: isMobile ? '.page-header' : '#nav-project'}, content: { title: "Save Project", description: "To finish the planning let's save our progress." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Project' });
        },}  },
      { attachTo: { element: '.text-input' }, content: { title: "Name your project", description: "Give you project a name." }, options: options},
      { attachTo: { element: '#save-project-row' }, content: { title: "Save your project", description: "Finally lets save your project for later." }, options: options},
      { attachTo: { element: '#map' }, content: { title: "First Project 🎉", description: "Congratulations, you planned your first line using OpenLinePlanner!\n\nFeel free to start exploring what else you can achieve with this tool.\n\n Lets improve public transport together!" }, options:mapMenuOptions },
    ];

    window.addEventListener('startTour', (e) => {
      start()
    })

    return {
      wrapper,
      steps,
      handleExit
    }
  }
})
</script>

<style lang="scss">
.tour-message {
  background-color: var(--c-background-primary);
  box-sizing: border-box;
  padding: $space-sm;
  border-radius: $br-md;
  //margin: $space-lg;
  max-width: 350px;

  &__head {
    widows: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;

    h3 {
      margin: 0;
    }
  }


  &__description {
    white-space: pre-line;
  }

  &__actions {
    widows: 100%;
    display: flex;
    justify-content: space-between;
    margin-top: $space-md;
  }
}
</style>

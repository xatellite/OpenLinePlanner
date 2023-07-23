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
      { attachTo: { element: isMobile ? '.mobile-nav' : '.page-header__navigation' }, content: { title: "Navigation", description: "In the top navigation bar you can switch between the different views for all the steps in your planning."}, options, on: {
        beforeStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Data' });
        },} },
      { attachTo: { element: '.data-list' }, content: { title: "Datasources", description: "To calculate the coverage of your planned lines, you need to load residential data for your planning area. You can view the currently loaded areas in the list on the left." }, options },
      { attachTo: { element: '#map' }, content: { title: "Add a new dataset", description: "Currently, no areas are loaded. \n To start loading your first dataset, klick on the map to select a municipality or district. \n Follow the popup on the bottom left to add the area to your planning dataset. \n By clicking on the map again, you can add as many areas as you need." }, options:mapMenuOptions },
      { attachTo: { element: '.data-list' }, content: { title: "Review your planning dataset", description: "You can now find all the areas you loaded over here. \n By clicking on one of them, you can visualize the loaded data in that area on the map. \n Let's continue to the planning view next." }, options: optionsMenu },
      { attachTo: { element: isMobile ? '.page-header' : '#nav-planning'}, content: { title: "Change the current view", description: "To begin drawing your lines, click \"Planning\" to change to the planning view." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Planning' });
        },} },
      { attachTo: { element: '.line-list' }, content: { title: "Create a new Line", description: "Click on \"Add new Line\" to create a new Line with the default configuration." }, options: optionsMenu},
      { attachTo: { element: '#map' }, content: { title: "Start drawing!", description: "Draw a line by clicking on the map to add points to your line. \n Once you are satisfied, hit \"enter\" to finish drawing. \n You can always click on a point of your line and drag it around to a new position, or click on the bin icon in the popup to delete it. \n You can also extend your line by clicking the pencil icon." }, options:mapMenuOptions },
      { attachTo: { element: '#map' }, content: { title: "Add stations", description: "The most important part of the planning is the placement of the stations. \n By clicking on a point on the line, and then clicking the station icon, you can create a new station. \n When you place a station, the station info popup opens automatically. \n This popup shows you informations about the coverage area, that is how many people live in the vicinity of this station. \n You can always get the popup back by clicking on the station again." }, options:mapMenuOptions },
      { attachTo: { element: '#view-settings' }, content: { title: "Analyse your planning", description: "In the setting view, you can enable various overlays to visualize the coverage and population in your planning area. Try it out by selecting \"distance\" for the coverage overlay, which shows you the households that are covered by your stations, and how far they are from the next one." }, options },
      { attachTo: { element: isMobile ? '.page-header' : '#nav-timetable'}, content: { title: "Check Timetable", description: "In the timetable view, you can review the travel times and stops along your line. \n Click on the timetable icon to open it." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Timetable' });
        },}  },
      { attachTo: { element: '.line-analysis' }, content: { title: "Travel time overview", description: "Here you can review you lines travel duration and distance, both in total and between the individual stops." }, options: options},
      { attachTo: { element: isMobile ? '.page-header' : '#nav-project'}, content: { title: "Save Project", description: "To finish the planning, let's go ahead and save our project so we can come back to it later." }, options, on: {
        afterStep: () => {
          app.appContext.config.globalProperties.$router.push({ name: 'Project' });
        },}  },
      { attachTo: { element: '.text-input' }, content: { title: "Name your project", description: "Give you project a name, so you can remember what you planned here." }, options: options},
      { attachTo: { element: '#save-project-row' }, content: { title: "Save your project", description: "Finally, click the save button to download the project file." }, options: options},
      { attachTo: { element: '#map' }, content: { title: "First Project 🎉", description: "Congratulations, you planned your first line using OpenLinePlanner!\n\nFeel free to start exploring the tool further and plan a few additional lines.\n\n Lets improve public transport together!" }, options:mapMenuOptions },
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

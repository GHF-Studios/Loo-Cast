using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Event
{
    using Core;
    
    public class EventListener : Component
    {
        public Event Event;
        public UnityEvent Response;

        private void OnEnable()
        {
            Event.RegisterListener(this);
        }

        private void OnDisable()
        {
            Event.UnregisterListener(this);
        }

        public void OnEventRaised()
        {
            Response.Invoke();
        }
    } 
}


export function validation(value:string | null,type:string,setError: React.Dispatch<React.SetStateAction<any>>):boolean{
    switch(type){
        case "username":
            if(value == null){
                setError((prev:{username:string,email:string,password:string})=>({...prev, username:"Atleast 4 characters with letters"}));
                return false
            }
            return /^(?=.*[a-zA-Z]).{4,}$/.test(value);
        case "email":
            if(value == null){
                setError((prev:{username:string,email:string,password:string})=>({...prev, email:"Please enter a valid email address"}));
                return false
            }
            return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
        case "password":
            if(value == null){  
                setError((prev:{username:string,email:string,password:string})=>({...prev, password:"Password cannot be empty and must be at least 6 characters"}));
                return false
            }
            return value.length >= 6;
        default:
            return false;
    }
}


// export function validateCondition()
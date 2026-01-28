import React, { useState } from "react";
import Input from "../store/Input";
import { X, Globe } from "lucide-react";
import { createWebsite } from '../../api/api';
import { useMutation, useQueryClient } from "@tanstack/react-query";

const Interval_option = [
    {label: "Every 1 minute(minimun)", value: 60},
    {label: "Every 2 minute", value: 120},
    {label: "Every 3 minute", value: 180},
    {label: "Every 4 minute", value: 240},
]

interface AddMonitorProps {
    onClose: () => void;
    onSuccess: () => void;
}

export default function AddMonitor({onClose}: AddMonitorProps) {

    const [payload, setPayLoad] = useState({
        url: "",
        name: "",
        interval: 60
    });

    const queryClient = useQueryClient();

    const addwebsiteMutation = useMutation ({
        mutationFn: createWebsite,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["website"]});
            onClose();
        },
        onError: () => {
            alert("Failed to add website");
        },
    });

    const [errors, setError] = useState ({
        name:"",
        url:""
    })

    const handleSubmit = async () => {
        
        const newErrors = {name: "", url: ""};
        let hasError = false;
        
        if(!payload.name.trim()) {
            newErrors.name = "Website Name is required";
            hasError = true;
        }
        if(!payload.url.trim()) {
            newErrors.url = "url is required";
            hasError = true;
        } else {
            try {
                new URL(payload.url)
            } catch (err){
                newErrors.url = "Please enter a valid URL...";
                hasError = true;
            }
        }
        if(hasError){
            setError(newErrors);
            return;
        }

        console.log("Adding Monitor:", payload);
        
        addwebsiteMutation.mutate({
            name: payload.name,
            url: payload.url,
            check_interval: payload.interval,
        });
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">

            <div className="w-full max-w-lg bg-card-bg border border-border-main rounded-xl shadow-2xl overflow-hidden animate-in fade-in zoom-in-95 duration-200">

                <div className="px-6 py-4 border-b border-border-main flex justify-between items-center bg-card-header/50">
                    <h2 className="text-lg font-semibold text-white flex items-center gap-2">
                        <Globe size={18} className="text-brand-blue" />
                        Add New Monitor
                    </h2>

                    <button onClick={onClose} className="text-slate-400 hover:text-white transition">
                        <X size={20} />
                    </button>
                </div>

                <div className="p-6 flex flex-col gap-5">
                    
                    <Input
                        label="Website Name"
                        inputName="name"
                        text={payload.name}
                        placeholder="e.g Google"
                        error={errors.name}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
                            setPayLoad({...payload, name: (e.target as HTMLInputElement).value as string});
                            if(errors.name) setError({...errors, name: ""});
                        }}
                        labelProps="text-sm font-medium text-slate-300 mb-1 block"
                        inputProp="w-full bg-input-bg border border-border-main rounded-lg px-3 py-2 text-white focus:ring-2 focus:ring-brand-blue outline-none transition"
                    />

                    <Input
                        label="URL"
                        inputName="url"
                        text={payload.url}
                        placeholder="https://google.com"
                        error={errors.url}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
                            setPayLoad({...payload, url: (e.target as HTMLInputElement).value as string});
                            if(errors.url) setError({...errors, url: ""});
                        }}
                        labelProps="text-sm font-medium text-slate-300 mb-1 block"
                        inputProp="w-full bg-input-bg border border-border-main rounded-lg px-3 py-2 text-white focus:ring-2 focus:ring-brand-blue outline-none transition"
                    />

                    <div className="flex flex-col gap-2">
                        <label className="text-sm font-medium text-slate-300 mb-1 block">Interval</label>
                        <div className="relative">
                            <select className="w-full bg-input-bg border border-border-main rounded-lg px-3 py-2 text-white focus:ring-2 focus:ring-brand-blue outline-none transition"
                                value={payload.interval}
                                onChange={(e) => setPayLoad({...payload, interval: Number(e.target.value)})}>
                                    {Interval_option.map((option) => (
                                        <option key={option.value} value={option.value}>
                                            {option.label}
                                        </option> 
                                    ))}
                            </select>
                        </div>
                    </div>
                </div>

                <div className="px-6 py-4 bg-card-header/50 border-t border-border-main flex justify-end gap-3">
                    <button onClick={onClose} className="px-4 py-2 text-sm font-medium text-slate-300 hover:text-white transition">
                        Cancel
                    </button>
                    <button onClick={handleSubmit} disabled={addwebsiteMutation.isPending} className="px-4 py-2 text-sm font-medium bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
                        {addwebsiteMutation.isPending? "Adding...": "Add Monitor"}
                    </button>
                </div>

            </div>
        </div>
    );
}

